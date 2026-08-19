# Multi-Agent Dashboards & Orchestration UI

---

## 1. Overview

Multi-agent applications orchestrate specialized sub-agents (e.g. Research Agent, Linter Agent, Synthesis Agent). The UI visualizes sub-agent execution progress as a Directed Acyclic Graph (DAG) or hierarchical step tree.

---

## 2. Multi-Agent DAG Hierarchy Diagram

```mermaid
graph TD
    Root[Orchestrator Agent] --> Agent1[Research Sub-Agent]
    Root --> Agent2[Code Index Sub-Agent]
    Agent1 --> Sub1[Search Web Engine]
    Agent2 --> Sub2[AST Parser Engine]
    Sub1 --> Synth[Synthesis Sub-Agent]
    Sub2 --> Synth
    Synth --> Final[Final Response Output]
```

---

## 3. Key References

- [LangGraph Agent Orchestration Framework](https://www.langchain.com/langgraph)
- [OpenAI Swarm Multi-Agent Architecture](https://github.com/openai/swarm)
