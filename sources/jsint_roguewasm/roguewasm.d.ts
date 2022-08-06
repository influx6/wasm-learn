/* tslint:disable */
export class Engine {
free(): void;

 constructor(arg0: any);

 on_dig(arg0: number, arg1: number, arg2: number): void;

 draw_map(): void;

 redraw_at(arg0: number, arg1: number): void;

 place_box(arg0: number, arg1: number): void;

 open_box(arg0: PlayerCore, arg1: number, arg2: number): void;

 mark_wasmprize(arg0: number, arg1: number): void;

 move_player(arg0: PlayerCore, arg1: number, arg2: number): void;

 free_cell(arg0: number, arg1: number): boolean;

}
export class PlayerCore {
free(): void;

 constructor(arg0: number, arg1: number, arg2: string, arg3: string, arg4: any);

 x(): number;

 y(): number;

 draw(): void;

 move_to(arg0: number, arg1: number): void;

 emit_stats(): void;

 take_damage(arg0: number): number;

}
