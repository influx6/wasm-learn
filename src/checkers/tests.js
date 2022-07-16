(async () => {
    const wasm = await fetch("./checkers.wat.wasm");
    const wasmBuffer = await wasm.arrayBuffer();
    const wasmModule = await WebAssembly.instantiate(wasmBuffer, {
        events: {
            piecemoved: (fx, fy, tx, ty) => {
                console.log("A piece moved from (%s, %s, %s, %s)", fx, fy, tx, ty);
            },
            piececrowned: (fx, fy) => {
                console.log("A piece has been crowned (%s, %s)", fx, fy);
            },
        }
    });

    console.log("Loaded wasm module", wasmModule);

    const wasmInstance = wasmModule.instance;
    const { initBoard, move } = wasmInstance.exports;

    console.log("Instance is ready: ", wasmInstance);

    initBoard();

    move(0,5,0,4);
    move(1,0,1,1);
})()