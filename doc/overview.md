```mermaid
---
title: Overview
---

flowchart TB

subgraph gui
    direction TB
    MainWindow --> Canvas
    MainWindow --> MainMenu
    
    Canvas --> Console
    MainMenu --> Console
    
    subgraph core
        direction TB

        subgraph data
            direction TB
            Command --> Session
            Session --> Project
            Project --> Layer
        end

        Console --> Command
        Command --> Service
        Service --> Session
        
        Session --> Tool
        Tool --> Layer
    end
    
    Service --> FreyaService
end

FreyaService --> Freya
FreyaService --> rfd

Command --> skia-safe
Layer --> skia-safe
```
