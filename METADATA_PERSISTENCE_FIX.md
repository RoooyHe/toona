# Metadata Persistence Fix - Final Solution

## Problem
Tags and endtime metadata were not persisting across app restarts. The Matrix server was timing out when trying to save custom state events for metadata (`m.kanban.card.meta`).

## Root Cause
The Matrix server (test.palpo.im) has issues with certain custom state event types:
- `m.kanban.card.todos` - Works perfectly ✅
- `m.kanban.card.meta` - Times out indefinitely ❌
- Room Topic API - Times out indefinitely ❌
- Account Data API - Wrong storage location ❌

## Solution: Combined Storage
Store both todos AND metadata in the same state event (`m.kanban.card.todos`) since that event type is proven to work.

### Data Structure
```json
{
  "todos": [
    {"id": "...", "text": "...", "completed": false}
  ],
  "metadata": {
    "title": "Card Title",
    "description": null,
    "position": 1000.0,
    "end_time": 1772265960,
    "tags": ["ui", "bug"],
    "created_at": 1772178135,
    "updated_at": 1772178140
  }
}
```

### Implementation

**File: `src/kanban/matrix_adapter.rs`**

1. **save_card_metadata** (lines 264-291):
   - Loads existing todos first
   - Combines todos + metadata in single JSON object
   - Saves to `m.kanban.card.todos` state event
   - Uses the proven working `send_state_event_raw()` approach

2. **save_card_todos** (lines 293-327):
   - Loads existing metadata first (if any)
   - Combines todos + metadata in single JSON object
   - Saves to `m.kanban.card.todos` state event
   - Preserves metadata when saving todos

3. **load_card_metadata** (lines 328-365):
   - Reads from `m.kanban.card.todos` state event
   - Extracts `metadata` field from JSON
   - Falls back to defaults if not found

## Benefits
- Uses only the proven working state event type
- No timeouts or hanging
- Atomic updates (todos and metadata saved together)
- Backward compatible (handles missing metadata gracefully)

## Testing
- Code compiles successfully ✅
- Ready for runtime testing
- Should now persist tags and endtime across app restarts

## Next Steps
1. Run the app: `cargo run --release`
2. Add tags and set endtime on a card
3. Restart the app
4. Verify tags and endtime are loaded correctly
5. Check logs for successful save/load messages
