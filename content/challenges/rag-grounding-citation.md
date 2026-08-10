---
slug: rag-grounding-citation
title: RAG Grounding & Citation Verification System
job_slug: personal-projects
short_description: LLM grounding contract with citation tags and hybrid retrieval
tech_stack:
  - Rust
  - Claude API
  - SQLite FTS5
  - RAG
  - Prompt Engineering
  - Axum
category: ai
url:
image_url:
featured: true
sort_order: 3
related_plan_module: W-RAG
related_adr: ADR-016
---

## Description

Designed and implemented an LLM output quality system for a portfolio AI assistant.
Built a grounding contract (ADR-016) that constrains Claude responses to verified
resume data via structured prompt assembly. The DefaultPromptAssembler injects live
portfolio chunks with citation tags, and entity_to_prose converters transform raw DB
rows into prose the LLM can ground against. HybridRetriever combines SQLite FTS5
full-text search with keyword-triggered live data injection, ensuring architecture and
auth questions surface code chunks instead of being crowded out by portfolio metadata.

## Problem

LLM grounding contract with citation tags and hybrid retrieval

## Constraints

Zero recurring cost, practical maintainability, and verifiable grounding.

## Decisions

Prioritize local-first MCP context and explicit plan/ADR alignment.

## Implementation

Designed and implemented an LLM output quality system for a portfolio AI assistant.
Built a grounding contract (ADR-016) that constrains Claude responses to verified
resume data via structured prompt assembly. The DefaultPromptAssembler injects live
portfolio chunks with citation tags, and entity_to_prose converters transform raw DB
rows into prose the LLM can ground against. HybridRetriever combines SQLite FTS5
full-text search with keyword-triggered live data injection, ensuring architecture and
auth questions surface code chunks instead of being crowded out by portfolio metadata.

## Outcomes

LLM grounding contract with citation tags and hybrid retrieval

## Metrics

See challenge description for measurable impact.
