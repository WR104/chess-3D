pub const WHITE_KING_POSITION_WEIGHTS: [[[f64; 5]; 5]; 5] = [
    [
        [-5.0, -5.0, -5.0, -5.0, -5.0],
        [-5.0, -5.0, -5.0, -5.0, -5.0],
        [-4.0, -5.0, -5.0, -4.0, -4.0],
        [-4.0, -5.0, -5.0, -5.0, -4.0],
        [-4.0, -4.0, -4.0, -4.0, -4.0],
    ],
    [
        [-5.0, -5.0, -5.0, -5.0, -5.0],
        [-5.0, -5.0, -5.0, -5.0, -5.0],
        [-4.0, -5.0, -5.0, -5.0, -4.0],
        [-3.0, -5.0, -5.0, -5.0, -3.0],
        [-3.0, -3.0, -3.0, -3.0, -3.0],
    ],
    [
        [-4.0, -4.0, -4.0, -4.0, -4.0],
        [-4.0, -4.0, -5.0, -4.0, -4.0],
        [-4.0, -4.0, -5.0, -4.0, -4.0],
        [-3.0, -3.0, -3.0, -3.0, -3.0],
        [-2.0, -2.0, -2.0, -2.0, -2.0],
    ],
    [
        [-3.0, -3.0, -3.0, -3.0, -3.0],
        [-2.0, -2.0, -2.0, -2.0, -2.0],
        [-1.0, -1.0, -1.0, -1.0, -1.0],
        [1.0, 1.0, 0.0, 1.0, 1.0],
        [1.0, 2.0, 1.0, 2.0, 1.0],
    ],
    [
        [-3.0, -3.0, -3.0, -3.0, -3.0],
        [-2.0, -2.0, -2.0, -2.0, -2.0],
        [-2.0, -1.0, -1.0, -1.0, -2.0],
        [1.0, 2.0, 0.0, 2.0, 1.0],
        [1.0, 2.0, 0.0, 2.0, 1.0],
    ],
];

pub const BLACK_KING_POSITION_WEIGHTS: [[[f64; 5]; 5]; 5] = [
    [
        [1.0, 2.0, 0.0, 2.0, 1.0],
        [1.0, 2.0, 0.0, 2.0, 1.0],
        [-2.0, -1.0, -1.0, -1.0, -2.0],
        [-2.0, -2.0, -2.0, -2.0, -2.0],
        [-3.0, -3.0, -3.0, -3.0, -3.0],
    ],
    [
        [1.0, 2.0, 1.0, 2.0, 1.0],
        [1.0, 1.0, 0.0, 1.0, 1.0],
        [-1.0, -1.0, -1.0, -1.0, -1.0],
        [-2.0, -2.0, -2.0, -2.0, -2.0],
        [-3.0, -3.0, -3.0, -3.0, -3.0],
    ],
    [
        [-2.0, -2.0, -2.0, -2.0, -2.0],
        [-3.0, -3.0, -3.0, -3.0, -3.0],
        [-4.0, -4.0, -5.0, -4.0, -4.0],
        [-4.0, -4.0, -5.0, -4.0, -4.0],
        [-4.0, -4.0, -4.0, -4.0, -4.0],
    ],
    [
        [-3.0, -3.0, -3.0, -3.0, -3.0],
        [-3.0, -5.0, -5.0, -5.0, -3.0],
        [-4.0, -5.0, -5.0, -5.0, -4.0],
        [-5.0, -5.0, -5.0, -5.0, -5.0],
        [-5.0, -5.0, -5.0, -5.0, -5.0],
    ],
    [
        [-4.0, -4.0, -4.0, -4.0, -4.0],
        [-5.0, -5.0, -5.0, -5.0, -4.0],
        [-5.0, -5.0, -5.0, -4.0, -4.0],
        [-5.0, -5.0, -5.0, -5.0, -5.0],
        [-5.0, -5.0, -5.0, -5.0, -5.0],
    ],
];

pub const WHITE_QUEEN_POSITION_WEIGHTS: [[[f64; 5]; 5]; 5] = [
    [
        [-2.0, -1.0, -1.0, -0.5, -0.5],
        [-1.0, 0.0, 0.0, 0.0, 0.0],
        [-1.0, 0.0, 0.5, 0.5, 0.5],
        [-0.5, 0.0, 0.5, 0.5, 0.5],
        [0.0, 0.0, 0.5, 0.5, 0.5],
    ],
    [
        [-1.0, 0.5, 0.5, 0.5, 0.5],
        [-1.0, 0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.5, 0.5],
        [-1.0, 0.5, 0.5, 0.5, 0.5],
        [-1.0, 0.0, 0.5, 0.0, 0.0],
    ],
    [
        [-1.0, -1.0, -0.5, -0.5, -0.5],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.5, 0.5, 0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5, 0.5, 0.5],
    ],
    [
        [0.5, 0.5, 0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5, 0.5, 0.5],
    ],
    [
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [-1.0, -0.5, -1.0, -0.5, -0.5],
        [-1.0, -0.5, -1.0, -0.5, -0.5],
        [-1.0, -0.5, -1.0, -0.5, -0.5],
        [-2.0, -1.0, -2.0, -1.0, -1.0],
    ],
];

pub const WHITE_UNICORN_POSITION_WEIGHTS: [[[f64; 5]; 5]; 5] = [
    [
        [-2.0, -1.5, -1.0, -1.5, -2.0],
        [-1.5, 0.0, 0.5, 0.0, -1.5],
        [-1.0, 0.5, 1.0, 0.5, -1.0],
        [-1.5, 0.0, 0.5, 0.0, -1.5],
        [-2.0, -1.5, -1.0, -1.5, -2.0]
    ],
    [
        [-1.5, 0.0, 0.5, 0.0, -1.5],
        [0.0, 0.5, 1.0, 0.5, 0.0],
        [0.5, 1.0, 1.5, 1.0, 0.5],
        [0.0, 0.5, 1.0, 0.5, 0.0],
        [-1.5, 0.0, 0.5, 0.0, -1.5]
    ],
    [
        [-1.0, 0.5, 1.0, 0.5, -1.0],
        [0.5, 1.0, 1.5, 1.0, 0.5],
        [1.0, 1.5, 2.0, 1.5, 1.0],
        [0.5, 1.0, 1.5, 1.0, 0.5],
        [-1.0, 0.5, 1.0, 0.5, -1.0]
    ],
    [
        [-1.5, 0.0, 0.5, 0.0, -1.5],
        [0.0, 0.5, 1.0, 0.5, 0.0],
        [0.5, 1.0, 1.5, 1.0, 0.5],
        [0.0, 0.5, 1.0, 0.5, 0.0],
        [-1.5, 0.0, 0.5, 0.0, -1.5]
    ],
    [
        [-2.0, -1.5, -1.0, -1.5, -2.0],
        [-1.5, 0.0, 0.5, 0.0, -1.5],
        [-1.0, 0.5, 1.0, 0.5, -1.0],
        [-1.5, 0.0, 0.5, 0.0, -1.5],
        [-2.0, -1.5, -1.0, -1.5, -2.0]
    ],
];


pub const WHITE_ROOK_POSITION_WEIGHTS: [[[f64; 5]; 5]; 5] = [
    [
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
    ],
    [
        [0.5, 1.0, 1.0, 1.0, 0.5],
        [0.0, 1.0, 1.0, 1.0, 0.0],
        [0.0, 1.0, 1.0, 1.0, 0.0],
        [0.0, 1.0, 1.0, 1.0, 0.0],
        [0.5, 0.0, 1.0, 1.0, 0.5],
    ],
    [
        [-0.5, 0.0, 0.0, 0.0, -0.5],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [-0.5, 0.0, 0.0, 0.0, -0.5],
    ],
    [
        [-0.5, 0.0, 0.0, 0.0, -0.5],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [-0.5, 0.0, 0.0, 0.0, -0.5],
    ],
    [
        [-0.5, 0.0, 0.0, 0.0, -0.5],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [-0.5, 0.0, 0.0, 0.0, -0.5],
    ],
];

pub const WHITE_BISHOP_POSITION_WEIGHTS: [[[f64; 5]; 5]; 5] = [
    [
        [-2.0, -1.0, -1.0, -1.0, -2.0],
        [-1.0, 0.0, 0.0, 0.0, -1.0],
        [-1.0, 0.0, 0.5, 0.0, -1.0],
        [-1.0, 0.0, 0.5, 0.0, -1.0],
        [-2.0, -1.0, -1.0, -1.0, -2.0],
    ],
    [
        [-1.0, 0.0, 0.0, 0.0, -1.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.5, 0.0],
        [0.0, 0.0, 0.5, 0.5, 0.0],
        [-1.0, 0.0, 0.0, 0.0, -1.0],
    ],
    [
        [-1.0, 0.0, 0.5, 0.5, -1.0],
        [0.0, 0.5, 0.5, 0.5, 0.0],
        [0.5, 0.5, 1.0, 0.5, 0.5],
        [0.0, 0.5, 0.5, 0.5, 0.0],
        [-1.0, 0.0, 0.5, 0.5, -1.0],
    ],
    [
        [-1.0, 0.0, 0.5, 0.5, -1.0],
        [0.0, 0.5, 0.5, 0.5, 0.0],
        [0.5, 0.5, 1.0, 0.5, 0.5],
        [0.0, 0.5, 0.5, 0.5, 0.0],
        [-1.0, 0.0, 0.5, 0.5, -1.0],
    ],
    [
        [-2.0, -1.0, -1.0, -1.0, -2.0],
        [-1.0, 0.0, 0.0, 0.0, -1.0],
        [-1.0, 0.0, 0.5, 0.0, -1.0],
        [-1.0, 0.0, 0.5, 0.0, -1.0],
        [-2.0, -1.0, -1.0, -1.0, -2.0],
    ],
];

pub const WHITE_KNIGHT_POSITION_WEIGHTS: [[[f64; 5]; 5]; 5] = [
    [
        [-5.0, -4.0, -3.0, -4.0, -5.0],
        [-4.0, -2.0, 0.0, -2.0, -4.0],
        [-3.0, 0.0, 1.0, 0.0, -3.0],
        [-3.0, 0.5, 1.5, 0.5, -3.0],
        [-3.0, 0.0, 1.5, 0.0, -3.0],
    ],
    [
        [-3.0, 0.5, 1.5, 0.5, -3.0],
        [0.5, 1.5, 2.0, 1.5, 0.5],
        [1.5, 2.0, 3.0, 2.0, 1.5],
        [1.0, 1.5, 2.0, 1.5, 1.0],
        [-2.0, -2.0, 0.0, -2.0, -2.0],
    ],
    [
        [-3.0, -1.0, -1.0, -1.0, -3.0],
        [0.0, 1.0, 1.0, 1.0, 0.0],
        [0.0, 1.0, 2.0, 1.0, 1.0],
        [0.5, 1.0, 2.0, 1.0, 0.5],
        [0.5, 1.0, 2.0, 1.0, 0.5],
    ],
    [
        [-3.0, 0.5, 1.5, 0.5, -3.0],
        [0.5, 1.5, 2.0, 1.5, 0.5],
        [0.5, 1.0, 2.0, 1.0, 0.5],
        [0.0, 0.5, 1.0, 0.5, 0.0],
        [-2.0, -2.0, 0.0, -2.0, -2.0],
    ],
    [
        [-3.0, 0.0, 1.5, 0.0, -3.0],
        [-3.0, 0.5, 1.5, 0.5, -3.0],
        [-3.0, 0.5, 1.5, 0.5, -3.0],
        [-4.0, -2.0, 0.0, -2.0, -4.0],
        [-5.0, -4.0, -3.0, -4.0, -5.0],
    ],
];


pub const WHITE_PAWN_POSITION_WEIGHTS: [[[f64; 5]; 5]; 5] = [
    [
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [5.0, 5.0, 5.0, 5.0, 5.0],
        [3.0, 4.0, 4.0, 4.0, 4.0],
        [2.0, 2.0, 2.0, 2.0, 2.0],
        [0.0, 0.0, 0.0, 0.0, 0.0]
    ],
    [
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [5.0, 5.0, 5.0, 5.0, 5.0],
        [3.0, 4.0, 4.0, 4.0, 3.0],
        [2.0, 2.0, 2.0, 2.0, 2.0],
        [0.0, 0.0, 0.0, 0.0, 0.0]
    ],
    [
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [1.0, 2.0, 3.0, 2.0, 1.0],
        [1.0, 3.0, 3.0, 3.0, 1.0],
        [1.0, 1.0, 1.0, 1.0, 1.0],
        [0.0, 0.0, 0.0, 0.0, 0.0]
    ],
    [
        [0.5, 0.5, 1.0, 0.5, 0.5],
        [0.5, 0.5, 1.0, 0.5, 0.5],
        [1.0, 1.0, 1.0, 1.0, 1.0],
        [-1.0, -1.0, -1.0, -1.0, -1.0],
        [0.0, 0.0, 0.0, 0.0, 0.0]
    ],
    [
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [-0.5, -0.5, 0.0, -0.5, -0.5],
        [0.0, 0.0, 0.0, 0.0, 0.0]
    ],
];
