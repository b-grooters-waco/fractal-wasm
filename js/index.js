import("../pkg/index.js")
    .then(wasm => {

        const cxStr = document.getElementById('real');
        const cyStr = document.getElementById('imaginary');
        const renderBtn = document.getElementById('render');
        const iterStr = document.getElementById('iterations');
        const zoomStr = document.getElementById('zoom');

        renderBtn.addEventListener('click', () => {
            const cx = parseFloat(cxStr.value) || 0;
            const cy = parseFloat(cyStr.value) || 0;
            const zoom = parseFloat(zoomStr.value) || 0;
            const iterations = parseInt(iterStr.value) || 0;
            const t0 = performance.now();
            wasm.render(480, 320, cx, cy, zoom, iterations);
            const t1 = performance.now();
            var message = "render elapsed time: " + (t1 - t0) + " milliseconds";
            console.log(message);
        });
        wasm.render(480, 320, -0.5, 0.0, 0.75, 100);
    })
    .catch(console.error);