const rust = import('../pkg/index.js');


rust.then(r => {
    r.run();
})
.catch(console.error);
