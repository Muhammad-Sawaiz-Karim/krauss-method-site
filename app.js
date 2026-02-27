import init, { generate_matrix_wasm } from './pkg/rust_port.js';

function parseInputString(inputString) {
    const stringArray = inputString.split(',');
    const numberArray = stringArray.map(str => parseInt(str.trim(), 10));
    const cleanNumbers = numberArray.filter(num => !isNaN(num));

    return new Int32Array(cleanNumbers);
}

function drawMatrix(matrixArray) {
    let latexString = "\\begin{bmatrix}\n";

    for (let i = 0; i < matrixArray.length; i++) {
        let row = matrixArray[i];
        let rowString = row.join(" & ");
        latexString += rowString + " \\\\\n";
    }

    latexString += "\\end{bmatrix}";

    const container = document.getElementById("matrix-container");

    katex.render(latexString, container, {
        displayMode: true,
        throwOnError: false
    });
}

async function run() {
    await init();
    console.log("WASM Loaded Successfully!");
    const generateButton = document.getElementById("generate_button");
    const rowInput = document.getElementById("row-input");
    const colInput = document.getElementById("col-input");
    const errorOutput = document.getElementById("error-output");
    const matrixContainer = document.getElementById("matrix-container");

    generateButton.addEventListener('click', () => {
        errorOutput.innerText = "";
        matrixContainer.innerHTML = "";

        const rowSums = parseInputString(rowInput.value);
        const colSums = parseInputString(colInput.value);

        try {
            const matrix = generate_matrix_wasm(rowSums, colSums);

            drawMatrix(matrix);
        }
        catch (error) {
            console.error("Error: ", error);
            errorOutput.innerText = error;
        }
    })
}
run();