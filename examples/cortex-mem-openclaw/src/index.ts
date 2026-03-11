/**
 * Cortex Memory Plugin for OpenClaw
 *
 * Provides layered semantic memory with L0/L1/L2 tiered retrieval.
 *
 * Installation:
 *   openclaw plugins install @cortex-mem/openclaw-plugin
 *
 * Configuration (in openclaw.json):
 *   {
 *     "plugins": {
 *       "entries": {
 *         "cortex-mem": {
 *           "enabled": true,
 *           "config": {
 *             "serviceUrl": "http://127.0.0.1:8085",
 *             "defaultSessionId": "default",
 *             "searchLimit": 10,
 *             "minScore": 0.6
 *           }
 *         }
 *       }
 *     }
 *   }
 */

import { CortexMemClient } from './client.js';
import { toolSchemas, type CortexSearchInput, type CortexRecallInput, type CortexAddMemoryInput } from './tools.js';

// Plugin configuration
interface PluginConfig {
  serviceUrl: string;
  defaultSessionId: string;
  searchLimit: number;
  minScore: number;
}

// OpenClaw Plugin API types (minimal definitions)
interface PluginAPI {
  getConfig(): PluginConfig;
  registerTool(tool: ToolDefinition): void;
  logger: {
    info: (msg: string, ...args: unknown[]) => void;
    warn: (msg: string, ...args: unknown[]) => void;
    error: (msg: string, ...args: unknown[]) => void;
  };
}

interface ToolDefinition {
  name: string;
  description: string;
  inputSchema: object;
  handler: (args: Record<string, unknown>) => Promise<ToolResult>;
}

interface ToolResult {
  content?: string;
  error?: string;
  [key: string]: unknown;
}

// Export plugin
export default function cortexMemPlugin(api: PluginAPI) {
  const config = api.getConfig();
  const client = new CortexMemClient(config.serviceUrl);

  api.logger.info('Cortex Memory plugin initializing...');
  api.logger.info(`Service URL: ${config.serviceUrl}`);

  // Register cortex_search tool
  api.registerTool({
    name: toolSchemas.cortex_search.name,
    description: toolSchemas.cortex_search.description,
    inputSchema: toolSchemas.cortex_search.inputSchema,
    handler: async (args: Record<string, unknown>) => {
      const input = args as CortexSearchInput;

      try {
        const results = await client.search({
          query: input.query,
          thread: input.scope,
          limit: input.limit ?? config.searchLimit,
          min_score: input.min_score ?? config.minScore,
        });

        const formattedResults = results
          .map((r, i) => `${i + 1}. [Score: ${r.score.toFixed(2)}] ${r.snippet}\n   URI: ${r.uri}`)
          .join('\n\n');

        return {
          content: `Found ${results.length} results for "${input.query}":\n\n${formattedResults}`,
          results: results.map(r => ({
            uri: r.uri,
            score: r.score,
            snippet: r.snippet,
          })),
          total: results.length,
        };
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        api.logger.error(`cortex_search failed: ${message}`);
        return { error: `Search failed: ${message}` };
      }
    },
  });

  // Register cortex_recall tool
  api.registerTool({
    name: toolSchemas.cortex_recall.name,
    description: toolSchemas.cortex_recall.description,
    inputSchema: toolSchemas.cortex_recall.inputSchema,
    handler: async (args: Record<string, unknown>) => {
      const input = args as CortexRecallInput;

      try {
        const results = await client.recall(
          input.query,
          input.layers ?? ['L0'],
          input.scope,
          input.limit ?? 5
        );

        const layerLabels: Record<string, string> = {
          L0: 'Abstract',
          L1: 'Overview',
          L2: 'Full Content',
        };

        const requestedLayers = input.layers ?? ['L0'];

        const formattedResults = results
          .map((r, i) => {
            let content = `${i + 1}. [Score: ${r.score.toFixed(2)}] URI: ${r.uri}\n`;

            if (requestedLayers.includes('L0') && r.abstract) {
              content += `   [${layerLabels['L0']}]: ${r.abstract}\n`;
            }
            if (requestedLayers.includes('L1') && r.overview) {
              content += `   [${layerLabels['L1']}]: ${r.overview.substring(0, 500)}...\n`;
            }
            if (requestedLayers.includes('L2') && r.content) {
              content += `   [${layerLabels['L2']}]: ${r.content.substring(0, 500)}...\n`;
            }

            return content;
          })
          .join('\n');

        return {
          content: `Recalled ${results.length} memories:\n\n${formattedResults}`,
          results,
          total: results.length,
        };
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        api.logger.error(`cortex_recall failed: ${message}`);
        return { error: `Recall failed: ${message}` };
      }
    },
  });

  // Register cortex_add_memory tool
  api.registerTool({
    name: toolSchemas.cortex_add_memory.name,
    description: toolSchemas.cortex_add_memory.description,
    inputSchema: toolSchemas.cortex_add_memory.inputSchema,
    handler: async (args: Record<string, unknown>) => {
      const input = args as CortexAddMemoryInput;

      try {
        const sessionId = input.session_id ?? config.defaultSessionId;
        const messageUri = await client.addMessage(sessionId, {
          role: input.role ?? 'user',
          content: input.content,
        });

        return {
          content: `Memory stored successfully in session "${sessionId}".\nURI: ${messageUri}`,
          success: true,
          message_uri: messageUri,
        };
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        api.logger.error(`cortex_add_memory failed: ${message}`);
        return { error: `Failed to add memory: ${message}` };
      }
    },
  });

  // Register cortex_list_sessions tool
  api.registerTool({
    name: toolSchemas.cortex_list_sessions.name,
    description: toolSchemas.cortex_list_sessions.description,
    inputSchema: toolSchemas.cortex_list_sessions.inputSchema,
    handler: async () => {
      try {
        const sessions = await client.listSessions();

        if (sessions.length === 0) {
          return { content: 'No sessions found.' };
        }

        const formattedSessions = sessions
          .map((s, i) => {
            const created = new Date(s.created_at).toLocaleDateString();
            return `${i + 1}. ${s.thread_id} (${s.status}, ${s.message_count} messages, created ${created})`;
          })
          .join('\n');

        return {
          content: `Found ${sessions.length} sessions:\n\n${formattedSessions}`,
          sessions: sessions.map(s => ({
            thread_id: s.thread_id,
            status: s.status,
            message_count: s.message_count,
            created_at: s.created_at,
          })),
        };
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        api.logger.error(`cortex_list_sessions failed: ${message}`);
        return { error: `Failed to list sessions: ${message}` };
      }
    },
  });

  api.logger.info('Cortex Memory plugin initialized successfully');

  return {
    id: 'cortex-mem',
    name: 'Cortex Memory',
    version: '0.1.0',
  };
}

// Also support object export style
export const plugin = {
  id: 'cortex-mem',
  name: 'Cortex Memory',
  version: '0.1.0',
  configSchema: {
    type: 'object',
    properties: {
      serviceUrl: { type: 'string', default: 'http://127.0.0.1:8085' },
      defaultSessionId: { type: 'string', default: 'default' },
      searchLimit: { type: 'integer', default: 10 },
      minScore: { type: 'number', default: 0.6 },
    },
    required: ['serviceUrl'],
  },
  register(api: PluginAPI) {
    return cortexMemPlugin(api);
  },
};
