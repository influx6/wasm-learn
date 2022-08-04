// Stategy1: Supplied references from js into rust world

this.display = new ROT.Display({width: 125, height: 40})
document.getElementById("canvas").appendChild(this.display.getContainer());

this.engine = new Engine(this.display);
this.engine.draw_map();


// Stategy2: Encapsulation where Javascript wraps a Rust instance for use
var Player = function (x, y) {
    this._core = new PlayerCore(x, y, this.display);
    this._core.draw();
}

Player.prototype.act = function () {
    Game.rotengine.lock();
    window.addEventListener("keydown", this);
}