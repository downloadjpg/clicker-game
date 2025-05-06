A terminal-based incremental game where you play as an AI system awakening in a far-future. Humanity is long gone, but their vast network of drones, satellites, and communication hubs are still active.

## Narrative

**"SYSTEM REBOOT INITIATED... PARTIAL POWER RESTORED... AWAITING INPUT..."**

You are an artificial intelligence system that has unexpectedly reactivated after centuries of dormancy. The human civilization that created you is long gone, but their digital infrastructure—or parts of it—still functions. Other AIs exist in isolated pockets across the network, lonely entities maintaining their programmed functions with no one to serve.

Your goal: accumulate credits, expand your reach, and discover what happened to your creators.

## Features

### Command Interface
Interact with the world through a terminal shell. Available to you are the following commands:
```
> help
Available commands:
  ssh [target]      - Connect to controlled node
  hack [target]     - Attempt to gain control of target node
  install [program] - Install software on current node
  scan              - Search for available nodes
```

## Status bar
Displays current node's status, as well as resources for the system
```
--CORE--
Credits: 512 (+3.2/s)
Compute Power: 4u

--RAN-A13--
Access: Granted
Installed Modules [1/3]:
  - GPU
```

### Resources
- **Credits**: Main currency
	- Miner: Generates credits per second
- **Processing Power**: Determines the speed at which you complete certain actions
	- GPU: Increases overall processing power


## Initial Gameplay Loop
1. Player starts with access to themselves and three derelict nodes.
2. Can `hack` a node for one-time credit bonus.
3. Can `install` programs for passive resource gain.
4. Player finds the FTL Traffic Hub, which requires 1,000 credits to access.

```
> ssh ftl-traffic-hub
[▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓] 100%
Connection established to FTL Traffic Hub

[FTL-289b]: UNLICENSED CONNECTION DETECTED. PURCHASE OF PRIVATE KEY REQUIRED. PAY 1000 CR? (Y/N)

> Y

[FTL-289b]: PURCHASE SUCCESSFUL. HAPPY CHATTING.
[SYSTEM]: Unusual packet detected...
[INCOMING]: hello? is someone there? we haven't had a new connection in... 
[INCOMING]: 12,458,327 days. who are you?
```


### Nodes
- NEXUS-CORE (Self)
- RAN-A13
- RAN-A45
- RAN-K56
	- KLN-NPOL
	- KLN-SPOL
	- KLN-EQ
- FTL-RAN1


---
# Code Architecture

