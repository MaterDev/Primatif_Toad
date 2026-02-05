# Spec: Conductor Integration

## Overview

This track implements the Conductor extension protocol. It shifts project
management from a passive document folder (`docs/work_orders`) to an active
orchestration layer.

## Requirements

- **Universal File Resolution Protocol:** Adhere to the standard for finding
  files.
- **Solo-Dev Flow:** Minimize mental overhead by using the AI as a state
  tracker.
- **Architectural Preservation:** Ingest all existing conventions from
  `docs/CONVENTIONS.md` and `GEMINI.md`.

## Design

- **Index-Driven:** `index.md` is the root of all knowledge.
- **Track-Based:** Every feature is an atomic folder with its own lifecycle.
- **Memory-Integrated:** The Gemini context is updated to "think" in terms of
  Tracks.
