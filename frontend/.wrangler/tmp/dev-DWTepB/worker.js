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

// node_modules/wrangler/templates/middleware/middleware-ensure-req-body-drained.ts
var drainBody = /* @__PURE__ */ __name(async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } finally {
    try {
      if (request.body !== null && !request.bodyUsed) {
        const reader = request.body.getReader();
        while (!(await reader.read()).done) {
        }
      }
    } catch (e) {
      console.error("Failed to drain the unused request body.", e);
    }
  }
}, "drainBody");
var middleware_ensure_req_body_drained_default = drainBody;

// node_modules/wrangler/templates/middleware/middleware-miniflare3-json-error.ts
function reduceError(e) {
  return {
    name: e?.name,
    message: e?.message ?? String(e),
    stack: e?.stack,
    cause: e?.cause === void 0 ? void 0 : reduceError(e.cause)
  };
}
__name(reduceError, "reduceError");
var jsonError = /* @__PURE__ */ __name(async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } catch (e) {
    const error2 = reduceError(e);
    return Response.json(error2, {
      status: 500,
      headers: { "MF-Experimental-Error-Stack": "true" }
    });
  }
}, "jsonError");
var middleware_miniflare3_json_error_default = jsonError;

// .wrangler/tmp/bundle-F0z6F3/middleware-insertion-facade.js
var __INTERNAL_WRANGLER_MIDDLEWARE__ = [
  middleware_ensure_req_body_drained_default,
  middleware_miniflare3_json_error_default
];
var middleware_insertion_facade_default = worker_default;

// node_modules/wrangler/templates/middleware/common.ts
var __facade_middleware__ = [];
function __facade_register__(...args) {
  __facade_middleware__.push(...args.flat());
}
__name(__facade_register__, "__facade_register__");
function __facade_invokeChain__(request, env, ctx, dispatch, middlewareChain) {
  const [head, ...tail] = middlewareChain;
  const middlewareCtx = {
    dispatch,
    next(newRequest, newEnv) {
      return __facade_invokeChain__(newRequest, newEnv, ctx, dispatch, tail);
    }
  };
  return head(request, env, ctx, middlewareCtx);
}
__name(__facade_invokeChain__, "__facade_invokeChain__");
function __facade_invoke__(request, env, ctx, dispatch, finalMiddleware) {
  return __facade_invokeChain__(request, env, ctx, dispatch, [
    ...__facade_middleware__,
    finalMiddleware
  ]);
}
__name(__facade_invoke__, "__facade_invoke__");

// .wrangler/tmp/bundle-F0z6F3/middleware-loader.entry.ts
var __Facade_ScheduledController__ = class ___Facade_ScheduledController__ {
  constructor(scheduledTime, cron, noRetry) {
    this.scheduledTime = scheduledTime;
    this.cron = cron;
    this.#noRetry = noRetry;
  }
  static {
    __name(this, "__Facade_ScheduledController__");
  }
  #noRetry;
  noRetry() {
    if (!(this instanceof ___Facade_ScheduledController__)) {
      throw new TypeError("Illegal invocation");
    }
    this.#noRetry();
  }
};
function wrapExportedHandler(worker) {
  if (__INTERNAL_WRANGLER_MIDDLEWARE__ === void 0 || __INTERNAL_WRANGLER_MIDDLEWARE__.length === 0) {
    return worker;
  }
  for (const middleware of __INTERNAL_WRANGLER_MIDDLEWARE__) {
    __facade_register__(middleware);
  }
  const fetchDispatcher = /* @__PURE__ */ __name(function(request, env, ctx) {
    if (worker.fetch === void 0) {
      throw new Error("Handler does not export a fetch() function.");
    }
    return worker.fetch(request, env, ctx);
  }, "fetchDispatcher");
  return {
    ...worker,
    fetch(request, env, ctx) {
      const dispatcher = /* @__PURE__ */ __name(function(type, init) {
        if (type === "scheduled" && worker.scheduled !== void 0) {
          const controller = new __Facade_ScheduledController__(
            Date.now(),
            init.cron ?? "",
            () => {
            }
          );
          return worker.scheduled(controller, env, ctx);
        }
      }, "dispatcher");
      return __facade_invoke__(request, env, ctx, dispatcher, fetchDispatcher);
    }
  };
}
__name(wrapExportedHandler, "wrapExportedHandler");
function wrapWorkerEntrypoint(klass) {
  if (__INTERNAL_WRANGLER_MIDDLEWARE__ === void 0 || __INTERNAL_WRANGLER_MIDDLEWARE__.length === 0) {
    return klass;
  }
  for (const middleware of __INTERNAL_WRANGLER_MIDDLEWARE__) {
    __facade_register__(middleware);
  }
  return class extends klass {
    #fetchDispatcher = /* @__PURE__ */ __name((request, env, ctx) => {
      this.env = env;
      this.ctx = ctx;
      if (super.fetch === void 0) {
        throw new Error("Entrypoint class does not define a fetch() function.");
      }
      return super.fetch(request);
    }, "#fetchDispatcher");
    #dispatcher = /* @__PURE__ */ __name((type, init) => {
      if (type === "scheduled" && super.scheduled !== void 0) {
        const controller = new __Facade_ScheduledController__(
          Date.now(),
          init.cron ?? "",
          () => {
          }
        );
        return super.scheduled(controller);
      }
    }, "#dispatcher");
    fetch(request) {
      return __facade_invoke__(
        request,
        this.env,
        this.ctx,
        this.#dispatcher,
        this.#fetchDispatcher
      );
    }
  };
}
__name(wrapWorkerEntrypoint, "wrapWorkerEntrypoint");
var WRAPPED_ENTRY;
if (typeof middleware_insertion_facade_default === "object") {
  WRAPPED_ENTRY = wrapExportedHandler(middleware_insertion_facade_default);
} else if (typeof middleware_insertion_facade_default === "function") {
  WRAPPED_ENTRY = wrapWorkerEntrypoint(middleware_insertion_facade_default);
}
var middleware_loader_entry_default = WRAPPED_ENTRY;
export {
  __INTERNAL_WRANGLER_MIDDLEWARE__,
  middleware_loader_entry_default as default
};
//# sourceMappingURL=worker.js.map
