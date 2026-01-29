# Scene Navigation Implementation Plan

## Overview
Implement scene navigation functionality using Bevy 0.18's States system.

## Target Files
- `src/main.rs` - Main file where all implementation takes place

## Implementation Details

### 1. AppState Definition
```rust
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    SceneList,       // Scene list screen (default on app start)
    SquirrelWalking, // Squirrel walking scene
    TestScene,       // Test scene
}
```

### 2. Scene List Screen
- Display title "Scene Selection" at center
- Two buttons: "Squirrel Walking", "Test Scene"
- Transition to respective scene on button click
- Color change effects on button hover/click

### 3. Scene Transition System
- `OnEnter(AppState::*)`: Spawn entities on scene entry
- `DespawnOnExitState`: Automatic entity cleanup on scene exit
- `.run_if(in_state(...))`: Conditional system execution per scene

### 4. Back Navigation
- Press ESC key to return to scene list
- Display "Press ESC to return" hint at top-left of each scene

### 5. Test Scene
- Display a circle with time-varying color (orange <-> blue)
- Show "Test Scene: Color Pulse Circle" description at bottom

## Implementation Steps

### Step 1: Basic Structure Changes
- [ ] Add `AppState` enum
- [ ] Add `.init_state::<AppState>()` to `main()`
- [ ] Change window title to "Bevy Playground"

### Step 2: Scene List UI Implementation
- [ ] Add UI color constants (NORMAL_BUTTON, HOVERED_BUTTON, etc.)
- [ ] Add `SceneButton` component
- [ ] Implement `setup_scene_list()` function (title + buttons)
- [ ] Implement `scene_button_interaction()` system

### Step 3: Squirrel Walking Scene Refactoring
- [ ] Rename existing `setup()` to `setup_squirrel_scene()`
- [ ] Add `DespawnOnExitState(AppState::SquirrelWalking)` to all entities
- [ ] Apply `.run_if(in_state(...))` to `move_player`, `animate_sprite`

### Step 4: Test Scene Implementation
- [ ] Add `ColorPulse` component
- [ ] Implement `setup_test_scene()` function
- [ ] Implement `test_scene_update()` system

### Step 5: Common Features
- [ ] Implement `back_to_menu()` system (ESC key handling)
- [ ] Implement `spawn_back_hint()` helper function

## Verification
1. Run `cargo run`
2. Verify scene list screen appears on app start
3. Click "Squirrel Walking" button and verify scene transition
4. Verify squirrel movement with WASD/arrow keys
5. Press ESC and verify return to scene list
6. Click "Test Scene" button and verify color-pulsing circle
7. Press ESC and verify return to scene list
8. Verify repeated scene transitions work correctly
