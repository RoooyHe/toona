# Todo Persistence Fix - Complete

## Problem
After restart, todos were not displaying in the card detail modal even though they were successfully saved to Matrix State Events.

## Root Cause
When `ShowCardDetail` action was triggered, it only loaded activities but didn't reload the card data from Matrix. The card data came from local cache which was populated during `ListLoaded`, but the cache didn't include todos after restart.

## Solution
Modified `ShowCardDetail` handler to reload the complete card data from Matrix before opening the modal:

1. **Added `LoadCard` request** to `MatrixRequest` enum:
   ```rust
   LoadCard {
       card_id: OwnedRoomId,
       space_id: OwnedRoomId,
   }
   ```

2. **Modified `ShowCardDetail` handler** in `src/app.rs`:
   - Now calls `MatrixRequest::LoadCard` to fetch fresh data from Matrix
   - This ensures todos are loaded from State Events every time the modal opens

3. **Implemented `LoadCard` handler** in `src/sliding_sync.rs`:
   - Calls `adapter.load_card()` which internally calls `load_card_todos()`
   - Posts `KanbanActions::CardLoaded` to update the in-memory state
   - The existing `CardLoaded` handler forces modal redraw if it's open

## Data Flow
1. User clicks card → `ShowCardDetail` action
2. `ShowCardDetail` triggers `MatrixRequest::LoadCard`
3. Background task calls `adapter.load_card()`
4. `load_card()` calls `load_card_metadata()` and `load_card_todos()`
5. Todos loaded from Matrix State Event `m.kanban.card.todos`
6. `CardLoaded` action updates in-memory state
7. Modal redraws with fresh data including todos

## Files Modified
- `src/app.rs`: Modified `ShowCardDetail` handler
- `src/sliding_sync.rs`: Added `LoadCard` variant and handler

## Testing
After restart:
1. Open any card with todos
2. Todos should display correctly in the modal
3. Check logs for "🔄 Reloading card" and "✅ Successfully loaded card: ... with N todos"
