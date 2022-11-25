// Description of the letter 'A'
//
// 0  - _______11_______
// 1  - ______1111______
// 2  - _____11__11_____
// 3  - ____11____11____
// 4  - ___11______11___
// 5  - __11________11__
// 6  - _11__________11_
// 7  - 11____________11
// 8  - 1111111111111111
// 9  - 11____________11
// 10 - 11____________11
// 11 - 11____________11
// 12 - 11____________11
// 13 - 11____________11
// 14 - 11____________11
// 15 - 11____________11
//
const A: [[bool; 16]; 16] = [
    // The first row of the letter
    [
        false, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, false,
    ],
    // The second row of the letter
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    // The third row of the letter
    [
        false, false, false, false, false, true, true, false, false, true, true, false, false,
        false, false, false,
    ],
    // The fourth row of the letter
    [
        false, false, false, false, true, true, false, false, false, false, true, true, false,
        false, false, false,
    ],
    // The fifth row of the letter
    [
        false, false, false, true, true, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    // The sixth row of the letter
    [
        false, false, true, true, false, false, false, false, false, false, false, false, true,
        true, false, false,
    ],
    // The seventh row of the letter
    [
        false, true, true, false, false, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    // The eight row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true,
    ],
    // The ninth row of the letter
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ],
    // The tenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true,
    ],
    // The eleventh row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true,
    ],
    // The twelfth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true,
    ],
    // The thirteenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true,
    ],
    // The fourteenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true,
    ],
    // The fifteenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true,
    ],
    // The sixteenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, true, true,
    ],
];

// Description of the letter 'B'
//
// 0  - 1111111111______
// 1  - 11________11____
// 2  - 11_________11___
// 3  - 11_________11___
// 4  - 11_________11___
// 5  - 11________11____
// 6  - 1111111111______
// 7  - 11________11____
// 8  - 11__________11__
// 9  - 11___________11_
// 10 - 11___________11_
// 11 - 11___________11_
// 12 - 11___________11_
// 13 - 11__________11__
// 14 - 11________11____
// 15 - 1111111111______
//
const B: [[bool; 16]; 16] = [
    // The first row of the letter
    [
        true, true, true, true, true, true, true, true, true, true, false, false, false, false,
        false, false,
    ],
    // The second row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, true, true, false,
        false, false, false,
    ],
    // The third row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    // The fourth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    // The fifth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    // The sixth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, true, true, false,
        false, false, false,
    ],
    // The seventh row of the letter
    [
        true, true, true, true, true, true, true, true, true, true, false, false, false, false,
        false, false,
    ],
    // The eighth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, true, true, false,
        false, false, false,
    ],
    // The ninth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, true,
        true, false, false,
    ],
    // The tenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    // The eleventh row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    // The twelfth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    // The thirteenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    // The fourteenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, false, false, true,
        true, false, false,
    ],
    // The fifteenth row of the letter
    [
        true, true, false, false, false, false, false, false, false, false, true, true, false,
        false, false, false,
    ],
    // The sixteenth row of the letter
    [
        true, true, true, true, true, true, true, true, true, true, false, false, false, false,
        false, false,
    ],
];

// Description of the letter 'C'
//
// 0  - ________________
// 1  - ______111111____
// 2  - ____11______11__
// 3  - ___11________11_
// 4  - __11_________11_
// 5  - __11____________
// 6  - __11____________
// 7  - __11____________
// 8  - __11____________
// 9  - __11____________
// 10 - __11____________
// 11 - __11____________
// 12 - __11_________11_
// 13 - ___11________11_
// 14 - ____11______11__
// 15 - ______111111____
//
const C: [[bool; 16]; 16] = [
    [false; 16],
    [
        false, false, false, false, false, false, true, true, true, true, true, true, false, false,
        false, false,
    ],
    [
        false, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false,
    ],
    [
        false, false, false, true, true, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    [
        false, false, false, true, true, false, false, false, false, false, false, false, false,
        true, true, false,
    ],
    [
        false, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, true, true, false, false,
        false, false,
    ],
];

// Description of the letter 'D'
//
// 0  - ________________
// 1  - 1111____________
// 2  - 11__11__________
// 3  - 11____11________
// 4  - 11______11______
// 5  - 11________11____
// 6  - 11_________11___
// 7  - 11_________11___
// 8  - 11_________11___
// 9  - 11_________11___
// 10 - 11_________11___
// 11 - 11________11____
// 12 - 11______11______
// 13 - 11____11________
// 14 - 11__11__________
// 15 - 1111____________
//
const D: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, true, true, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, true, true, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, true, true, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, true, true, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, true, true, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, true, true, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, true, true, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, true, true, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
];

// Description of the letter 'E'
//
// 0  - ________________
// 1  - 11111111111111__
// 2  - 11111111111111__
// 3  - 11______________
// 4  - 11______________
// 5  - 11______________
// 6  - 11111111111_____
// 7  - 11111111111_____
// 8  - 11______________
// 9  - 11______________
// 10 - 11______________
// 11 - 11______________
// 12 - 11______________
// 13 - 11______________
// 14 - 11111111111111__
// 15 - 11111111111111__
//
const E: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
];

// Description of the letter 'F'
//
// 0  - ________________
// 1  - 11111111111111__
// 2  - 11111111111111__
// 3  - 11______________
// 4  - 11______________
// 5  - 11______________
// 6  - 11111111111_____
// 7  - 11111111111_____
// 8  - 11______________
// 9  - 11______________
// 10 - 11______________
// 11 - 11______________
// 12 - 11______________
// 13 - 11______________
// 14 - 11______________
// 15 - 11______________
//
const F: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
];

// Description of the letter 'G'
//
// 0  - ________________
// 1  - ____111111______
// 2  - __11______11____
// 3  - 11_________11___
// 4  - 11_________11___
// 5  - 11______________
// 6  - 11______________
// 7  - 11______________
// 8  - 11___1111111____
// 9  - 11___1111111____
// 10 - 11_________11___
// 11 - 11_________11___
// 12 - 11_________11___
// 13 - 11_________11___
// 14 - __11______11____
// 15 - ____111111______
//
const G: [[bool; 16]; 16] = [
    [false; 16],
    [
        false, false, false, false, true, true, true, true, true, true, false, false, false, false,
        false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, false, false, false, true, true, true, true, true, true, true, false, false,
        false, false,
    ],
    [
        true, true, false, false, false, true, true, true, true, true, true, true, false, false,
        false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, true, false, false, false, false,
        false, false,
    ],
];

// Description of the letter 'H'
//
// 0  - ________________
// 1  - 11_________11___
// 2  - 11_________11___
// 3  - 11_________11___
// 4  - 11_________11___
// 5  - 11_________11___
// 6  - 1111111111111___
// 7  - 1111111111111___
// 8  - 11_________11___
// 9  - 11_________11___
// 10 - 11_________11___
// 11 - 11_________11___
// 12 - 11_________11___
// 13 - 11_________11___
// 14 - 11_________11___
// 15 - 11_________11___
//
const H: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
    [
        true, true, false, false, false, false, false, false, false, false, false, true, true,
        false, false, false,
    ],
];

// Description of the letter 'I'
//
// 0  - ________________
// 1  - 1111111111111___
// 2  - 1111111111111___
// 3  - ____11111_______
// 4  - ____11111_______
// 5  - ____11111_______
// 6  - ____11111_______
// 7  - ____11111_______
// 8  - ____11111_______
// 9  - ____11111_______
// 10 - ____11111_______
// 11 - ____11111_______
// 12 - ____11111_______
// 13 - ____11111_______
// 14 - 1111111111111___
// 15 - 1111111111111___
//
const I: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
];

// Description of the letter 'J'
//
// 0  - ________________
// 1  - 11111111111_____
// 2  - 11111111111_____
// 3  - ______111_______
// 4  - ______111_______
// 5  - ______111_______
// 6  - ______111_______
// 7  - ______111_______
// 8  - ______111_______
// 9  - ______111_______
// 10 - ______111_______
// 11 - ______111_______
// 12 - ______111_______
// 13 - 111___111_______
// 14 - _111__111_______
// 15 - ___1111_________
//
const J: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, false, false, false,
        false, false,
    ],
    [
        false, true, true, true, false, false, true, true, true, false, false, false, false, false,
        false, false,
    ],
    [
        false, false, false, true, true, true, true, false, false, false, false, false, false,
        false, false, false,
    ],
];

// Description of the letter 'K'
//
// 0  - ________________
// 1  - 111_____111_____
// 2  - 111____111______
// 3  - 111___111_______
// 4  - 111_111_________
// 5  - 11111___________
// 6  - 111_111_________
// 7  - 111__111________
// 8  - 111___111_______
// 9  - 111____111______
// 10 - 111_____111_____
// 11 - 111______111____
// 12 - 111_______111___
// 13 - 111________111__
// 14 - 111_________111_
// 15 - 111_________111_
//
const K: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, true, true, true, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, true, true, true, false, false, false, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, true, true, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, true, true, true, false, false, false, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, true, true, true, false, false, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, true, true, true, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, true, true, true, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
];

// Description of the letter 'L'
//
// 0  - ________________
// 1  - 111_____________
// 2  - 111_____________
// 3  - 111_____________
// 4  - 111_____________
// 5  - 111_____________
// 6  - 111_____________
// 7  - 111_____________
// 8  - 111_____________
// 9  - 111_____________
// 10 - 111_____________
// 11 - 111_____________
// 12 - 111_____________
// 13 - 111_____________
// 14 - 1111111111111___
// 15 - 1111111111111___
//
const L: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
];

// Description of the letter 'M'
//
// 0  - ________________
// 1  - 111_________111_
// 2  - 11111_____11111_
// 3  - 111_111_111_111_
// 4  - 111___111___111_
// 5  - 111___111___111_
// 6  - 111___111___111_
// 7  - 111____1____111_
// 8  - 111_________111_
// 9  - 111_________111_
// 10 - 111_________111_
// 11 - 111_________111_
// 12 - 111_________111_
// 13 - 111_________111_
// 14 - 111_________111_
// 15 - 111_________111_
//
const M: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, true, true, false, false, false, false, false, true, true, true, true,
        true, false,
    ],
    [
        true, true, true, false, true, true, true, false, true, true, true, false, true, true,
        true, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, false, true, true,
        true, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, false, true, true,
        true, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, false, true, true,
        true, false,
    ],
    [
        true, true, true, false, false, false, false, true, false, false, false, false, true, true,
        true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
];

// Description of the letter 'N'
//
// 0  - ________________
// 1  - 111________111__
// 2  - 111________111__
// 3  - 1111_______111__
// 4  - 11111______111__
// 5  - 111_111____111__
// 6  - 111___111__111__
// 7  - 111____111_111__
// 8  - 111______11111__
// 9  - 111_______1111__
// 10 - 111________111__
// 11 - 111________111__
// 12 - 111________111__
// 13 - 111________111__
// 14 - 111________111__
// 15 - 111________111__
//
const N: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, true, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, true, true, true, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, true, true, true, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, true, true, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, true, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
];

// Description of the letter 'O'
//
// 0  - ________________
// 1  - _____11111______
// 2  - ___111111111____
// 3  - __111_____111___
// 4  - _111_______111__
// 5  - 111_________111_
// 6  - 111_________111_
// 7  - 111_________111_
// 8  - 111_________111_
// 9  - 111_________111_
// 10 - 111_________111_
// 11 - _111_______111__
// 12 - __111_____111___
// 13 - ___111___111____
// 14 - ____1111111_____
// 15 - _____11111______
//
const O: [[bool; 16]; 16] = [
    [false; 16],
    [
        false, false, false, false, false, true, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, true, true, true, true, true, true, true, true, true, false, false,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        false, true, true, true, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        false, true, true, true, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        false, false, false, true, true, true, false, false, false, true, true, true, false, false,
        false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, true, false, false, false,
        false, false, false,
    ],
];

// Description of the letter 'P'
//
// 0  - ________________
// 1  - 1111111111111___
// 2  - 11111111111111__
// 3  - 111________111__
// 4  - 111________111__
// 5  - 111_______111___
// 6  - 11111111111_____
// 7  - 111111111_______
// 8  - 111_____________
// 9  - 111_____________
// 10 - 111_____________
// 11 - 111_____________
// 12 - 111_____________
// 13 - 111_____________
// 14 - 111_____________
// 15 - 111_____________
//
const P: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, false, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
];

// Description of the letter 'Q'
//
// 0  - ________________
// 1  - _____111111_____
// 2  - ___1111__1111___
// 3  - __111______111__
// 4  - __111______111__
// 5  - __111______111__
// 6  - __111______111__
// 7  - __111______111__
// 8  - __111______111__
// 9  - __111_111__111__
// 10 - __111__111_111__
// 11 - ___111_111_111__
// 12 - ____11111111____
// 13 - ________111_____
// 14 - _________111____
// 15 - ___________111__
const Q: [[bool; 16]; 16] = [
    [false; 16],
    [
        false, false, false, false, false, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        false, false, false, true, true, true, true, false, false, true, true, true, true, false,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, true, true, true, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, true, true, true, false, true, true, true,
        false, false,
    ],
    [
        false, false, false, true, true, true, false, true, true, true, false, true, true, true,
        false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, true, true, true, false, false,
        false, false,
    ],
    [
        false, false, false, false, false, false, false, false, true, true, true, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, true, true, true, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, true, true,
        true, false, false,
    ],
];

// Description of the letter 'R'
//
// 0  - ________________
// 1  - 1111111111111___
// 2  - 11111111111111__
// 3  - 111________111__
// 4  - 111________111__
// 5  - 111_______111___
// 6  - 11111111111_____
// 7  - 111111111_______
// 8  - 111____111______
// 9  - 111_____111_____
// 10 - 111______111____
// 11 - 111_______111___
// 12 - 111________111__
// 13 - 111_________111_
// 14 - 111_________111_
// 15 - 111_________111_
//
const R: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, false, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, true, true, true, false, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, true, true, true, false, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, true, true, true, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
];

// Description of the letter 'S'
//
// 0  - ________________
// 1  - _____1111111____
// 2  - ___111_____111__
// 3  - __111_______111_
// 4  - __111_______111_
// 5  - ___111__________
// 6  - ____111_________
// 7  - _____111________
// 8  - _______111______
// 9  - _________111____
// 10 - __________111___
// 11 - __111______111__
// 12 - __111______111__
// 13 - __111______111__
// 14 - ____111____111__
// 15 - _____1111111____
//
const S: [[bool; 16]; 16] = [
    [false; 16],
    [
        false, false, false, false, false, true, true, true, true, true, true, true, false, false,
        false, false,
    ],
    [
        false, false, false, true, true, true, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        false, false, false, true, true, true, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, false, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, true, true, true, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, true, true, true,
        false, false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, false, false, true, true, true, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, true, true, true, false, false,
        false, false,
    ],
];

// Description of the letter 'T'
//
// 0  - ________________
// 1  - 111111111111111_
// 2  - 111111111111111_
// 3  - ______1111______
// 4  - ______1111______
// 5  - ______1111______
// 6  - ______1111______
// 7  - ______1111______
// 8  - ______1111______
// 9  - ______1111______
// 10 - ______1111______
// 11 - ______1111______
// 12 - ______1111______
// 13 - ______1111______
// 14 - ______1111______
// 15 - ______1111______
//
const T: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false,
    ],
];

// Description of the letter 'U'
//
// 0  - ________________
// 1  - 111_________111_
// 2  - 111_________111_
// 3  - 111_________111_
// 4  - 111_________111_
// 5  - 111_________111_
// 6  - 111_________111_
// 7  - 111_________111_
// 8  - 111_________111_
// 9  - 111_________111_
// 10 - 111_________111_
// 11 - 111_________111_
// 12 - _111_______111__
// 13 - __111_____111___
// 14 - ____1111111_____
// 15 - ________________
//
const U: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        false, true, true, true, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, true, true, false, false, false,
        false, false,
    ],
    [false; 16],
];

// Description of the letter 'V'
//
// 0   - ________________
// 1   - 111_________111_
// 2   - 111_________111_
// 3   - 111_________111_
// 4   - 111_________111_
// 5   - 111_________111_
// 6   - 111_________111_
// 7   - 111_________111_
// 8   - 111_________111_
// 9   - 111_________111_
// 10  - 111_________111_
// 11  - _111_______111__
// 12  - __111_____111___
// 13  - ___111___111____
// 14  - ____111_111_____
// 15  - ______111_______
//
const V: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        false, true, true, true, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        false, false, false, true, true, true, false, false, false, true, true, true, false, false,
        false, false,
    ],
    [
        false, false, false, false, true, true, true, false, true, true, true, false, false, false,
        false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
];

// Description of the letter 'W'
//
// 0  - ________________
// 1  - 111_________111_
// 2  - 111_________111_
// 3  - 111_________111_
// 4  - 111_________111_
// 5  - 111_________111_
// 6  - 111_________111_
// 7  - 111_________111_
// 8  - 111_________111_
// 9  - 111___111___111_
// 10 - 111___111___111_
// 11 - 111_111_111_111_
// 12 - 111111___111111_
// 13 - 11111_____11111_
// 14 - 1111_______1111_
// 15 - 111_________111_
//
const W: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, false, true, true,
        true, false,
    ],
    [
        true, true, true, false, false, false, true, true, true, false, false, false, true, true,
        true, false,
    ],
    [
        true, true, true, false, true, true, true, false, true, true, true, false, true, true,
        true, false,
    ],
    [
        true, true, true, true, true, true, false, false, false, true, true, true, true, true,
        true, false,
    ],
    [
        true, true, true, true, true, false, false, false, false, false, true, true, true, true,
        true, false,
    ],
    [
        true, true, true, true, false, false, false, false, false, false, false, true, true, true,
        true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
];

// Description of the letter 'X'
//
// 0  - ________________
// 1  - 111_________111_
// 2  - 111_________111_
// 3  - 111_________111_
// 4  - _111_______111__
// 5  - __111_____111___
// 6  - ___111___111____
// 7  - ____111_111_____
// 8  - _____11111______
// 9  - _____1111_______
// 10 - ___111_111______
// 11 - __111___111_____
// 12 - _111_____111____
// 13 - 111_______111___
// 14 - 111________111__
// 15 - 111_________111_
//
const X: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
    [
        false, true, true, true, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        false, false, false, true, true, true, false, false, false, true, true, true, false, false,
        false, false,
    ],
    [
        false, false, false, false, true, true, true, false, true, true, true, false, false, false,
        false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, true, true, true, false, true, true, true, false, false, false, false,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, true, true, true, false, false, false,
        false, false,
    ],
    [
        false, true, true, true, false, false, false, false, false, true, true, true, false, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        true, true, false,
    ],
];

// Description of the letter 'Y'
//
// 0  - ________________
// 1  - 111________111__
// 2  - _111______111___
// 3  - __111____111____
// 4  - ___111__111_____
// 5  - ____111111______
// 6  - _____1111_______
// 7  - _____1111_______
// 8  - _____1111_______
// 9  - _____1111_______
// 10 - _____1111_______
// 11 - _____1111_______
// 12 - _____1111_______
// 13 - _____1111_______
// 14 - _____1111_______
// 15 - _____1111_______
//
const Y: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, false,
    ],
    [
        false, true, true, true, false, false, false, false, false, false, true, true, true, false,
        false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, true, true, true, false, false,
        false, false,
    ],
    [
        false, false, false, true, true, true, false, false, true, true, true, false, false, false,
        false, false,
    ],
    [
        false, false, false, false, true, true, true, true, true, true, false, false, false, false,
        false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, true, false, false, false, false,
        false, false, false,
    ],
];

// Description of the letter 'Z'
//
// 0  - ________________
// 1  - 11111111111111__
// 2  - 11111111111111__
// 3  - ___________111__
// 4  - __________111___
// 5  - _________111____
// 6  - ________111_____
// 7  - _______111______
// 8  - ______111_______
// 9  - _____111________
// 10 - ____111_________
// 11 - ___111__________
// 12 - __111___________
// 13 - _111____________
// 14 - 11111111111111__
// 15 - 11111111111111__
//
const Z: [[bool; 16]; 16] = [
    [false; 16],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, true, true,
        true, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, true, true, true,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, true, true, true, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, true, true, true, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, false, true, true, true, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, false, true, true, true, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, false, true, true, true, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, false, true, true, true, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, false, true, true, true, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, false, true, true, true, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        false, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
        false,
    ],
];

pub const FONT: [[[bool; 16]; 16]; 26] = [
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
];
