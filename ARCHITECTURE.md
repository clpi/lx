# Architecture

## About

### Modes
- There are four modes availabile to switch between immediately at any given time, through the `CTRL` plus `z` through `v` keys on the bottom row. Might consider making these keys more ergonomic, but the most common mode-switches (i.e. switching between edit <-> insert) are intended to be done through other means.
	1. **Insert**: `<c-v>` The default, starting mode, where keypresses directly manipulate the focused buffer.
		- **Context**:
		 	_none at the moment_ ^movement/edit history/register?^

	3. **Edit**: `<c-c>` The mode most comparable to Vim's Normal mode, where `hjkl` moves the cursor arround the buffer, and various text movement/selection mechanisms are available through different keypresses (more comparable to kakoune than vim).
		- **Context**:
			 _none at the moment_   ^movement/edit history/register?^

	4. **Command**: `<c-x>` Issue commands like in Vim or Emacs etc. No funny business here unless I think of anything. Goal will be to try and make the command palette omnibar-esque like a FZF or VS code interface, rather than straight command string greps.
		- **Context**:
			- `cmd_buf`: `String`: buffer of currently inputted command

	5. **Overview**: `<c-x>` Where you accomplish everything other than strict code editing, while still in a "editing" session. This involves buffer/session/file management, directory navigation/manipulation, and also provding a platform for extensibility for other users to add modules onto.
		- **Context**:
			- `view_idx`: `usize`: currently focused view in overview


---
## Architecture

### Event Types
- Three main event types
	1. **Mode switch**: The key press/mouseclick switches the mode of the editor and immediately changes the view and event recognition context. No effect on following events otherwise. Available in any mode, however, the "trigger" mode switch (`ESC` and `CR` currently) has an action dependent on the current mode of the edtor.
	3. **Prefix**: Key press enables a remembered trigger dictating the effects of certain following keypresses. Prefix key events are essentially the roots of trees, whose leaves correspond to a single operation of a corresponding operation table.
		1. **Global prefix**: The key press enables a prefix trigger which dictates the action of the following keypress(es) _for any mode_. Typically, such global prefixes are reserved for non mode-specific actions (i.e. opening up a file finder interface or switching a window/buffer view)
		2. **Mode prefix**: The keypress is matched for in one specific mode, and dictates the actions of certain following key events in that mode only. Reserved for most mode-specific actions.
	4. **Single-press actions**: Actions which execute immediately upon being matched, if and only if no prefix trigger is currently active. Can be mode-specific ((w)ord or (b)ackwards navigation/selection in edit mode) or global.

### Event loop

- The event loop is handled by matching for different key event categories in this order:
	1. Editor context:
		- `prefix` : `Optional key` _inits as None_
		- `mode` : `Mode(ModeContext)` in `Insert`, `Edit`, `Overview`, `Command` _inits as Insert_
		- `buf_idx`: usize _inits as index of singly open buffer_
		- `buffers`: `Vec<String>`  _inits with one string in vec_ ^note: dont use a string^
	2. User passes input, read as event datatype `E_0`
	3. Currently, loop only checks for case where `E_0` is key event, that is `K_0`
	4. If last key press `K_-1` was some prefix key trigger `P` with operation enum `O`:
		1. Match `K_0` for matching `P`-subordinate key press patterns `[Pp_1..Pp_k]`
		2. If got match `Pp_n` (that is, `K_0 == Pp_n`):
			1. execute corresponding operation `O_n`, then continue to base step 1 of next loop
		3. If no match (`K_0 != Pp_n` _for all_ `[Pp_1 .. Pp_k]`):
			1. Continue to step 4 of this base loop
	5. Check if `K_0` matches mode-switching keys `[Ss_1..Ss_k]`
		1. If `K_0 == Ss_n` then switch mode to `M_n` corresponding to `Ss_n`. Then continue to step 1 of next loop.
		2. Otherwise, do nothing, continue to base step 6 of this loop.
	6. For current mode `M` with set of valid prefix key events `[PMpm_1 .. PMpm_j]`, and single press key events `[SMsm_1 .. SMsm_k]`, _(note: includes BS, CR for insert mode)_
		1. Check if `K_0` has prefix mode keyevent match `PMpm_n`
			2. If got match `K_0 == PMpm_n`, set editor context `prefix = Some(P)` and continue to step 1 of next loop
			3. Otherwise, continue to check for single press mode key events `SMsm_n`
		2. Check if `K_0` has single press mode keyevent match `SMsm_n`
			3. If got match `K_0 = SMsm_n`, execute op `Osm_n` associated with the mode keypress
			4. Otherwise, continue to step 7 of base loop
	7. For set of valid global prefix key events `[PGg_1 .. PGg_k]` and single press global keyevents `[SGsg_1 .. SGsg_j]`
		1. Check if `K_0` has prefix mode keyevent match `PMpm_n`
			2. If got match `K_0 == PMpm_n`, set editor context `prefix = Some(P)` and continue to step 1 of next loop
			3. Otherwise, continue to check for single press mode key events `SMsm_n`
		2. Check if `K_0` has single press mode keyevent match `SMsm_n`
			3. If got match `K_0 = SMsm_n`, execute op `Osm_n` associated with the mode keypress
			4. Otherwise, continue to step 7 of base loop
	8. Having exhausted all defined global and mode local key events (prefix and singlepress), check mode:
		1. If current mode `M` = `Insert`, and `K_0` has keycode that is alphanumeric, append `K_0` to current buffer
		1. If current mode `M` = `Command`, and `K_0` has keycode that is alphanumeric, append `K_0` to current buffer

---
### Default Maps

#### Mode Switching
- `<Ctrl+z>`: Switch to **Overview Mode**. ^[any mode]^
- `<Ctrl+x>`: Switch to **Command Mode**. ^[any mode]^
- `<Ctrl+c>`: Switch to **Edit mode**. ^[any mode]^
- `<Ctrl+v>`: Switch to **Insert mode** _(default)_. ^[any mode]^
- `<ESC>`: Behavior dependent on mode:
	- ^[edit mode]^ Switch to **Overview Mode**
	- ^[insert mode]^ ^[command mode]^ ^[overview mode]^ Switch to **Edit Mode**
- `<CR>`: Behavior dependent on mode:
	- ^[edit mode]^ Switch to **Insert Mode**
	- ^[overview mode]^ Switch to most recent **Edit mode**
	- ^[insert mode]^ _No mode-switching effect_
	- ^[command mode]^ _Executes command, then returns to previous mode_


---
### Notes
- ^[07/19/21]^ ^[05:25]^ Might switch precedence of modeswitch cmds and prefix commands (as shown in this document)? Currently prefix takes higher precedence than mode switching. Might keep it that way.
- ^[07/19/21]^ ^[05:28]^ Might consider having mode switching branch out from one 'mode switch' global prefix command -- like ctrl+space? Maybe...
- ^[07/19/21]^ ^[05:30]^ Would like to map Ctrl+Enter to toggle modes. Hard to do with crossterm?
