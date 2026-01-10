use check_buddy::piece_type::PieceType;
use check_buddy::BoardMap;
use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use std::hint::black_box;

#[library_benchmark]
fn single_move() {
    let board = BoardMap::starting();
    black_box(board.gen_to_positions(black_box([0, 0])));
}

#[library_benchmark]
fn opponent_moves() {
    let board = BoardMap::starting();
    black_box(board.gen_all_opponent_positions());
}

#[library_benchmark]
fn all_moves() {
    let board = BoardMap::starting();
    black_box(
        (0..8)
            .flat_map(|rank| {
                (0..8)
                    .map(|file| board.gen_to_positions(black_box([rank, file])))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
}

#[library_benchmark]
fn all_legal_moves() {
    let board = BoardMap::starting();
    black_box(
        (0..8)
            .flat_map(|rank| {
                (0..8)
                    .map(|file| board.gen_legal_positions(black_box([rank, file])))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
}

#[library_benchmark]
fn pawn() {
    let board = BoardMap::starting();
    black_box(board.gen_pawn(black_box([1, 0])));
}

#[library_benchmark]
fn king() {
    let board = BoardMap::starting();
    black_box(board.gen_king(black_box([0, 4])));
}

#[library_benchmark]
fn knight() {
    let board = BoardMap::starting();
    black_box(board.gen_knight(black_box([0, 1])));
}

#[library_benchmark]
fn queen() {
    let board = BoardMap::starting();
    black_box(board.gen_sliding(black_box([0, 1]), PieceType::Queen));
}

library_benchmark_group!(
    name = generate_moves;
    benchmarks = single_move, opponent_moves, all_moves, all_legal_moves
);

library_benchmark_group!(
    name = generate_piece_moves;
    benchmarks = pawn, king, knight, queen
);

main!(
    library_benchmark_groups = generate_moves,
    generate_piece_moves
);
