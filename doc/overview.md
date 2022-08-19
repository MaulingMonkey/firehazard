| Security Feature      | Parent | Child | Notes |
| --------------------- | ------ | ----- | ----- |
| Enable SIDs           | ✔️ | ❌ |
| Restrict SIDs         | ✔️ | ❌ |
| Privileges            | ✔️ | ✔️ | Remove from token, do not merely disable
| Integrity             | ⚠️ | ✔️ | Lowering process post-spawn token from parent seems fragile in practice
| Policies              | ❌ | ✔️ | Currently incomplete
| Desktop / Winsta      | ✔️ | ?  | Maybe if I perform shenannigans and close my old "current process/thread" handles?
| Console               | ✔️ | ?  | AttachConsole can potentially reattach?
| Environment Variables | ✔️ | ❌ |
| Job                   | ✔️ | ? |
