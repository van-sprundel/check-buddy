use check_buddy_core::piece_type::PieceType;
use check_buddy_core::BoardMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn generate_moves(c: &mut Criterion) {
    let board = BoardMap::starting();
    let mut group = c.benchmark_group("generate_moves");
    group.bench_function("single_move", |b| {
        b.iter(|| board.gen_to_positions(black_box([0, 0])))
    });
    group.bench_function("opponent_moves", |b| {
        b.iter(|| board.gen_all_opponent_positions())
    });
    group.bench_function("all_moves", |b| {
        b.iter(|| {
            (0..8).flat_map(|rank| {
                (0..8)
                    .map(|file| board.gen_to_positions(black_box([rank, file])))
                    .collect::<Vec<_>>()
            })
        })
    });
    group.bench_function("all_legal_moves", |b| {
        b.iter(|| {
            (0..8).flat_map(|rank| {
                (0..8)
                    .map(|file| board.gen_legal_positions(black_box([rank, file])))
                    .collect::<Vec<_>>()
            })
        })
    });
    group.finish();
}

fn generate_piece_moves(c: &mut Criterion) {
    let board = BoardMap::starting();
    let mut group = c.benchmark_group("generate_piece_moves");
    group.bench_function("pawn", |b| b.iter(|| board.gen_pawn(black_box([1, 0]))));
    group.bench_function("king", |b| b.iter(|| board.gen_king(black_box([0, 4]))));
    group.bench_function("knight", |b| b.iter(|| board.gen_knight(black_box([0, 1]))));
    group.bench_function("queen", |b| {
        b.iter(|| board.gen_sliding(black_box([0, 1]), PieceType::Queen))
    });
}
criterion_group!(benches, generate_moves, generate_piece_moves);
criterion_main!(benches);
