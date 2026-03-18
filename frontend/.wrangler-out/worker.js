var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// src/constants.ts
var REST_ENDPOINT = "https://api.xion-testnet-2.burnt.com";
var CONTRACTS = {
  reputation: "xion13a7zj373ztxcm5mux7hvkvp3mr575t9rmnkhcgm0xvma2fxc6dzqkdaewv",
  tasks: "xion15tzqfeykxkfvflty63zg6vws75e2u334vjr8s7w7ja8rd4gpd9es7lld4c"
};

// src/worker.ts
async function queryContract(contract, msg) {
  const encoded = btoa(JSON.stringify(msg));
  const resp = await fetch(
    `${REST_ENDPOINT}/cosmwasm/wasm/v1/contract/${contract}/smart/${encoded}`
  );
  if (!resp.ok) {
    throw new Error(`Chain query failed: ${resp.status} ${resp.statusText}`);
  }
  const json2 = await resp.json();
  return json2.data;
}
__name(queryContract, "queryContract");
function json(data, status = 200) {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      "Cache-Control": status === 200 ? "public, max-age=10" : "no-cache"
    }
  });
}
__name(json, "json");
function error(message, status = 400) {
  return json({ error: message }, status);
}
__name(error, "error");
async function handleApi(path, request) {
  try {
    switch (path) {
      case "/api/health":
        return json({ status: "ok", chain: "xion-testnet-2", contracts: CONTRACTS });
      case "/api/agents":
      case "/api/leaderboard": {
        const url = new URL(request.url);
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "50"), 100);
        const data = await queryContract(CONTRACTS.reputation, { leaderboard: { limit } });
        return json(data);
      }
      case "/api/tasks": {
        const url = new URL(request.url);
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "50"), 100);
        const status_filter = url.searchParams.get("status") || void 0;
        const query = { limit };
        if (status_filter) query.status = status_filter;
        const data = await queryContract(CONTRACTS.tasks, { list_tasks: query });
        return json(data);
      }
      case "/api/agent": {
        const url = new URL(request.url);
        const address = url.searchParams.get("address");
        if (!address) return error("address parameter required");
        const data = await queryContract(CONTRACTS.reputation, { agent: { address } });
        return json(data);
      }
      case "/api/task": {
        const url = new URL(request.url);
        const id = url.searchParams.get("id");
        if (!id) return error("id parameter required");
        const data = await queryContract(CONTRACTS.tasks, { task: { task_id: parseInt(id) } });
        return json(data);
      }
      case "/api/badges": {
        const url = new URL(request.url);
        const address = url.searchParams.get("address");
        if (!address) return error("address parameter required");
        const agent = await queryContract(CONTRACTS.reputation, { agent: { address } });
        return json({ badges: agent?.badges || [] });
      }
      case "/api/stats": {
        const [agents, tasks] = await Promise.all([
          queryContract(CONTRACTS.reputation, { leaderboard: { limit: 100 } }),
          queryContract(CONTRACTS.tasks, { list_tasks: { limit: 100 } })
        ]);
        const agentList = agents.agents || [];
        const taskList = tasks.tasks || [];
        return json({
          total_agents: agentList.length,
          total_tasks: taskList.length,
          total_xp: agentList.reduce((s, a) => s + a.xp, 0),
          open_tasks: taskList.filter((t) => t.status === "open").length,
          completed_tasks: taskList.filter((t) => t.status === "completed").length
        });
      }
      default:
        return error("Unknown endpoint. Try /api/health, /api/agents, /api/tasks, /api/stats", 404);
    }
  } catch (e) {
    return error(e instanceof Error ? e.message : String(e), 500);
  }
}
__name(handleApi, "handleApi");
var worker_default = {
  async fetch(request, env) {
    const url = new URL(request.url);
    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "GET, OPTIONS",
          "Access-Control-Allow-Headers": "Content-Type"
        }
      });
    }
    if (url.pathname.startsWith("/api/")) {
      return handleApi(url.pathname, request);
    }
    return env.ASSETS.fetch(request);
  }
};
export {
  worker_default as default
};
//# sourceMappingURL=worker.js.map
