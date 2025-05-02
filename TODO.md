IN PROGRESS

- [ ] mvp items list
    - [ ] crypto miner - gives you 0.05 credits per second
    - [ ] hack-multiplier - increases hacking power by +0.5x
    - [ ] gpu - increases efficiency of miners by +0.2x
    - [ ] retirement fund - victory!
    - [ ] status window - watch paint dry, real time!
    - [ ] shop window - call that window shopping


- Item (trait? struct?) with a callback for modifying game state on purchase
game state holds list of items owned
also holds list of items in shop?

BACKLOG:
- [ ] make proper input pathway
    - look at the menu examples in docs

- command history

- alias system?
    - each command has alias attribute, command checks against all aliases for matching?
    - global 'alias' list, pass at beginning and replace if match
    - same runtime, i think

- maybe i should redesign the whole 'gamestate' thing to be more modular... have data for each

DONE
- [x] status panel
- [x] make blinking cursor 
- [x] clean up init code (read ratatui docs)


Event handling options:
- each module handles events on their own
- events passed to module
