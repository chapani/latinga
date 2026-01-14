# Using Latinga with WebAssembly (WASM)

Latinga can be integrated into high-performance web applications or Node.js environments. By utilizing the stateful ```Latinga```, you can reuse heavy dictionary structures across multiple conversion calls.

## Installation
First, build the package using ```wasm-pack```:
```bash
wasm-pack build --features wasm
```

## Integration Patterns

### 1. Browser (ES Modules)
```javascript
import init, { Latinga } from './pkg/latinga.js';

async function start() {
    await init();
    
    // Initialize once (Heavy operation)
    // Constructor argument: false = New Latin (Kelgusi), true = Current (Joriy)
    const latinga = new Latinga(false);

    // Add custom logic
    latinga.almashuvchilarni_yukla("Microsoft=>Maykrosoft");
    latinga.atoqlilarni_yukla("Rust,Wasm");
    latinga.qalqonlarni_yukla("\\[.*?\\]"); // Protect text in brackets

    // Convert (High-speed operation)
    const output = latinga.ogir("Rust Londonda.");
    console.log(output); 
}
```

### 2. Node.js
If you built for Node.js (```wasm-pack build --target nodejs```):

```javascript
const { Latinga } = require('./pkg/latinga.js');

const latinga = new Latinga(true); 
console.log(latinga.ogir("кирилл")); 
```

## Performance Note
The ```Latinga``` is **stateful**. Create one instance at the start of your application life cycle and reuse it.
