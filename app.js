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

function drawGraph(matrixArray) {
    const elements = [];
    const rowCount = matrixArray.length;
    const colCount = matrixArray[0].length;

    for (let i = 0; i < rowCount; i++) {
        elements.push({
            data: {id: `R${i}`, label: `R${i}`},
            position: {x: 100, y: i * 80 + 50},
            classes: 'row-node'
        });
    }

    for (let i = 0; i < colCount; i++) {
        elements.push({
            data: {id: `C${i}`, label: `C${i}`},
            position: {x: 400, y: i * 80 + 50},
            classes: 'col-node'
        });
    }

    for (let i = 0; i < rowCount; i++) {
        for (let j = 0; j < colCount; j++) {
            if (matrixArray[i][j] == 1) {
                elements.push({
                    data: {
                        id: `E-R${i}-C${j}`,
                        source: `R${i}`,
                        target: `C${j}`
                    }
                });
            }
        }
    }

    cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        layout: {name: 'preset',
            fit: true,
            padding: 50
        },
        userZoomingEnabled: true,
        userPanningEnabled: true,

        style: [
            {
                selector: 'node',
                style: {
                    'label': 'data(label)',
                    'text-valign': 'center',
                    'color': '#fff',
                    'font-size': '12px',
                    'text-outline-width': 1,
                    'text-outline-color': '#333'
                }
            },

            {
                selector: '.row-node',
                style: { 'background-color': '#0074D9', 'shape': 'triangle', 'width': 60 }
            },

            {
                selector: '.col-node',
                style: { 'background-color': '#FF4136', 'shape': 'circle', 'width': 60 }
            },

            {
                selector: 'edge',
                style: {
                    'width': 2,
                    'line-color': '#aaa',
                    'curve-style': 'bezier'
                }
            }
        ]
    });
}

async function run() { 
    try {
        await init('pkg/rust_port_bg.wasm'); 
        console.log("WASM Loaded!");
    } catch (e) {
        console.error("Initialization failure:", e);
        alert("MIME/Load Error: " + e.message);
        return;
    }
    
    const generateButton = document.getElementById("generate_button");
    const rowInput = document.getElementById("row-input");
    const colInput = document.getElementById("col-input");
    const errorOutput = document.getElementById("error-output");
    const matrixContainer = document.getElementById("matrix-container");

    generateButton.addEventListener('click', () => {
        errorOutput.innerText = "";
        matrixContainer.innerHTML = "";
        cy.innerHTML = "";

        const rowSums = parseInputString(rowInput.value);
        const colSums = parseInputString(colInput.value);

        try {
            const matrix = generate_matrix_wasm(rowSums, colSums);

            drawMatrix(matrix);
            drawGraph(matrix);
        }
        catch (error) {
            console.error("Error: ", error);
            errorOutput.innerText = error;
        }
    })
}
run();
