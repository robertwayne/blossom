use blossom::{entity::EntityId, quickmap::QuickMap, room::Room, vec3::Vec3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};

use std::collections::HashMap;

fn bench_comparisons(c: &mut Criterion) {
    let x = 1000;
    let y = 1000;
    let z = 1;

    let rooms_vec = generate_rooms_vec(x, y, z);
    let rooms_hashmap = generate_rooms_hashmap(x, y, z);
    let rooms_quickmap = generate_rooms_quickmap(x, y, z);

    let mut rng = thread_rng();
    let random_positions = (0..100)
        .map(|_| Vec3::new(rng.gen_range(-x..x), rng.gen_range(-y..y), rng.gen_range(-z..z)))
        .collect::<Vec<_>>();

    let mut group_get = c.benchmark_group("get");
    group_get.bench_function("bench_rooms_vec_get", |b| {
        b.iter(|| {
            for pos in random_positions.iter() {
                black_box(rooms_vec.iter().find(|r| r.position == *pos));
            }
        })
    });

    group_get.bench_function("bench_rooms_hashmap_get", |b| {
        b.iter(|| {
            for pos in random_positions.iter() {
                black_box(rooms_hashmap.get(&*pos));
            }
        })
    });

    group_get.bench_function("bench_rooms_quickmap_get", |b| {
        b.iter(|| {
            for pos in random_positions.iter() {
                black_box(rooms_quickmap.get(&*pos));
            }
        })
    });
    group_get.finish();

    let mut group_iter = c.benchmark_group("iter");
    group_iter.bench_function("bench_rooms_vec_iterate", |b| {
        b.iter(|| {
            for room in rooms_vec.iter() {
                black_box(room);
            }
        })
    });

    group_iter.bench_function("bench_rooms_hashmap_iterate", |b| {
        b.iter(|| {
            for (_, room) in rooms_hashmap.iter() {
                black_box(room);
            }
        })
    });

    group_iter.bench_function("bench_rooms_quickmap_iterate", |b| {
        b.iter(|| {
            for room in rooms_quickmap.iter() {
                black_box(room);
            }
        })
    });
    group_iter.finish();
}

fn generate_rooms_vec(x: i32, y: i32, z: i32) -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();

    for x_val in 0..x {
        for y_val in 0..y {
            for z_val in -z..z {
                let room = Room {
                    entity_id: EntityId::default(),
                    mob_pool: Vec::new(),
                    name: format!("{}-{}-{}", x_val, y_val, z_val),
                    position: Vec3::new(x_val, y_val, z_val),
                    description: "This is a room.".to_string(),
                    exits: Vec::new(),
                };

                rooms.push(room);
            }
        }
    }

    rooms
}

fn generate_rooms_hashmap(x: i32, y: i32, z: i32) -> HashMap<Vec3, Room> {
    let mut rooms: HashMap<Vec3, Room> = HashMap::new();

    for x_val in 0..x {
        for y_val in 0..y {
            for z_val in -z..z {
                let room = Room {
                    entity_id: EntityId::default(),
                    mob_pool: Vec::new(),
                    name: format!("{}-{}-{}", x_val, y_val, z_val),
                    position: Vec3::new(x_val, y_val, z_val),
                    description: "This is a room.".to_string(),
                    exits: Vec::new(),
                };

                rooms.insert(room.position, room);
            }
        }
    }

    rooms
}

fn generate_rooms_quickmap(x: i32, y: i32, z: i32) -> QuickMap<Vec3, Room> {
    let mut rooms: QuickMap<Vec3, Room> = QuickMap::new();

    for x_val in 0..x {
        for y_val in 0..y {
            for z_val in -z..z {
                let room = Room {
                    entity_id: EntityId::default(),
                    mob_pool: Vec::new(),
                    name: format!("{}-{}-{}", x_val, y_val, z_val),
                    position: Vec3::new(x_val, y_val, z_val),
                    description: "This is a room.".to_string(),
                    exits: Vec::new(),
                };

                rooms.insert(room);
            }
        }
    }

    rooms
}

criterion_group!(benches, bench_comparisons);
criterion_main!(benches);
