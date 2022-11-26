use criterion::{criterion_group,black_box, criterion_main, Criterion};
use check_buddy_core::board::BoardMap;

fn generate_move(c: &mut Criterion) {
    let board = BoardMap::starting();
    c.bench_function("generate_single_move", |b| b.iter(||  board.gen_moves(black_box([0,0]))));
}

fn generate_moves(c: &mut Criterion) {
    let board = BoardMap::starting();
    c.bench_function("generate_all_moves", |b| {
        b.iter_with_setup(|| (0..8).flat_map(|rank| (0..8).map(|file| board.gen_moves(black_box([rank,file]))).collect::<Vec<_>>()), |v| v);
    });
}

fn generate_legal_moves(c: &mut Criterion) {
    let board = BoardMap::starting();
    c.bench_function("generate_all_legal_moves", |b| {
        b.iter_with_setup(|| (0..8).flat_map(|rank| (0..8).map(|file| board.gen_legal_moves(black_box([rank,file]))).collect::<Vec<_>>()), |v| v)
    });
}

criterion_group!(benches, generate_move, generate_moves, generate_legal_moves);
criterion_main!(benches);