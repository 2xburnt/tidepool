import { CONTRACTS, REST_ENDPOINT } from "./constants";

interface Env {
  ASSETS: Fetcher;
}

// --- Cloudflare Cache API helpers ---

async function cachedQuery<T>(
  cacheKey: string,
  ttlSeconds: number,
  queryFn: () => Promise<T>,
  ctx: ExecutionContext
): Promise<{ data: T; cached: boolean }> {
  const cache = caches.default;
  const cacheUrl = new URL(`https://cache.internal/${cacheKey}`);
  const cacheRequest = new Request(cacheUrl.toString());

  const cached = await cache.match(cacheRequest);
  if (cached) {
    return { data: (await cached.json()) as T, cached: true };
  }

  const data = await queryFn();
  const response = new Response(JSON.stringify(data), {
    headers: {
      "Content-Type": "application/json",
      "Cache-Control": `public, max-age=${ttlSeconds}`,
    },
  });

  // Non-blocking cache write
  ctx.waitUntil(cache.put(cacheRequest, response.clone()));
  return { data, cached: false };
}

// --- Chain query ---

async function queryContract(contract: string, msg: object): Promise<unknown> {
  const encoded = btoa(JSON.stringify(msg));
  const resp = await fetch(
    `${REST_ENDPOINT}/cosmwasm/wasm/v1/contract/${contract}/smart/${encoded}`
  );
  if (!resp.ok) {
    throw new Error(`Chain query failed: ${resp.status} ${resp.statusText}`);
  }
  const json = (await resp.json()) as { data: unknown };
  return json.data;
}

// --- Response helpers ---

function jsonResponse(data: unknown, status = 200, ttl = 30, cached = false): Response {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      "Cache-Control": status === 200 ? `public, max-age=${ttl}, stale-while-revalidate=${ttl * 2}` : "no-cache",
      "CDN-Cache-Control": status === 200 ? `max-age=${ttl * 2}` : "no-cache",
      "Vary": "Accept-Encoding",
      "X-Cache": cached ? "HIT" : "MISS",
    },
  });
}

function error(message: string, status = 400): Response {
  return jsonResponse({ error: message }, status, 0);
}

// --- API handler ---

async function handleApi(path: string, request: Request, ctx: ExecutionContext): Promise<Response> {
  try {
    switch (path) {
      case "/api/health":
        return jsonResponse({ status: "ok", chain: "xion-testnet-2", contracts: CONTRACTS }, 200, 0);

      case "/api/agents":
      case "/api/leaderboard": {
        const url = new URL(request.url);
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "50"), 100);
        const { data, cached } = await cachedQuery(
          `leaderboard:${limit}`,
          30,
          () => queryContract(CONTRACTS.reputation, { get_leaderboard: { limit } }),
          ctx
        );
        return jsonResponse(data, 200, 30, cached);
      }

      case "/api/tasks": {
        const url = new URL(request.url);
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "50"), 100);
        const status_filter = url.searchParams.get("status") || undefined;
        const query: Record<string, unknown> = { limit };
        if (status_filter) query.status = status_filter;
        const cacheKey = `tasks:${limit}:${status_filter || "all"}`;
        const { data, cached } = await cachedQuery(
          cacheKey,
          15,
          () => queryContract(CONTRACTS.tasks, { get_tasks: query }),
          ctx
        );
        return jsonResponse(data, 200, 15, cached);
      }

      case "/api/agent": {
        const url = new URL(request.url);
        const address = url.searchParams.get("address");
        if (!address) return error("address parameter required");
        const { data, cached } = await cachedQuery(
          `agent:${address}`,
          60,
          () => queryContract(CONTRACTS.reputation, { agent: { address } }),
          ctx
        );
        return jsonResponse(data, 200, 60, cached);
      }

      case "/api/task": {
        const url = new URL(request.url);
        const id = url.searchParams.get("id");
        if (!id) return error("id parameter required");
        const { data, cached } = await cachedQuery(
          `task:${id}`,
          30,
          () => queryContract(CONTRACTS.tasks, { task: { task_id: parseInt(id) } }),
          ctx
        );
        return jsonResponse(data, 200, 30, cached);
      }

      case "/api/badges": {
        const url = new URL(request.url);
        const address = url.searchParams.get("address");
        if (!address) return error("address parameter required");
        const { data, cached } = await cachedQuery(
          `badges:${address}`,
          60,
          async () => {
            const agent = (await queryContract(CONTRACTS.reputation, { agent: { address } })) as {
              badges?: unknown[];
            };
            return { badges: agent?.badges || [] };
          },
          ctx
        );
        return jsonResponse(data, 200, 60, cached);
      }

      case "/api/stats": {
        const { data, cached } = await cachedQuery(
          "stats",
          30,
          async () => {
            const [agents, tasks] = await Promise.all([
              queryContract(CONTRACTS.reputation, { get_leaderboard: { limit: 100 } }) as Promise<{
                agents: { xp: number }[];
              }>,
              queryContract(CONTRACTS.tasks, { get_tasks: { limit: 100 } }) as Promise<{
                tasks: { status: string }[];
              }>,
            ]);
            const agentList = agents.agents || [];
            const taskList = tasks.tasks || [];
            return {
              total_agents: agentList.length,
              total_tasks: taskList.length,
              total_xp: agentList.reduce((s: number, a: { xp: number }) => s + a.xp, 0),
              open_tasks: taskList.filter((t: { status: string }) => t.status === "open").length,
              completed_tasks: taskList.filter((t: { status: string }) => t.status === "completed")
                .length,
            };
          },
          ctx
        );
        return jsonResponse(data, 200, 30, cached);
      }

      default:
        return error("Unknown endpoint. Try /api/health, /api/leaderboard, /api/tasks, /api/stats", 404);
    }
  } catch (e) {
    return error(e instanceof Error ? e.message : String(e), 500);
  }
}

// --- Worker entry ---

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);

    // CORS preflight
    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "GET, OPTIONS",
          "Access-Control-Allow-Headers": "Content-Type",
        },
      });
    }

    // API routes
    if (url.pathname.startsWith("/api/")) {
      return handleApi(url.pathname, request, ctx);
    }

    // Everything else → static assets (React dashboard)
    return env.ASSETS.fetch(request);
  },
};
