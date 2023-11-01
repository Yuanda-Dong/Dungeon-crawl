use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    let fov = <&FieldOfView>::query().filter(component::<Player>()).iter(ecs).nth(0).unwrap();

    <(&Point, &Render)>::query().iter(ecs)
        .filter(|(pos,_)| fov.visible_tiles.contains(pos))
        .for_each(|(p, r)| {
            draw_batch.set(*p - offset, r.color, r.glyph);
        });
    draw_batch.submit(5000).expect("Batch error");
}
