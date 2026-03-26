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

    // 1. Create Row Nodes (Top)
    for (let i = 0; i < rowCount; i++) {
        elements.push({
            data: { id: `R${i}`, label: `R${i}` },
            classes: 'row-node'
        });
    }

    // 2. Create Column Nodes (Bottom)
    for (let i = 0; i < colCount; i++) {
        elements.push({
            data: { id: `C${i}`, label: `C${i}` },
            classes: 'col-node'
        });
    }

    // 3. Create Edges
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

    // 4. Initialize Cytoscape with the Modern Style
    cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        
        // Using 'grid' with 2 rows puts R nodes on top and C nodes on bottom automatically
        layout: {
            name: 'grid',
            rows: 2,
            padding: 50,
            spacingFactor: 1.1
        },

        style: [
            {
                selector: 'node',
                style: {
                    'label': 'data(label)',
                    'text-valign': 'center',
                    'text-halign': 'center',
                    'color': '#fff',
                    'font-size': '12px',
                    'font-family': 'monospace',
                    'width': '40px',
                    'height': '40px',
                    'border-width': '2px',
                    'border-color': '#fff'
                }
            },
            {
                selector: '.row-node',
                style: { 
                    'background-color': '#4f46e5', // Modern Indigo
                    'shape': 'round-rectangle' 
                }
            },
            {
                selector: '.col-node',
                style: { 
                    'background-color': '#10b981', // Crisp Emerald
                    'shape': 'ellipse' 
                }
            },
            {
                selector: 'edge',
                style: {
                    'width': 2,
                    'line-color': '#2f3236', // Light slate gray
                    'curve-style': 'bezier',
                    'target-arrow-shape': 'triangle', // Optional: adds direction
                    'target-arrow-color': '#2f3236',
                    'opacity': 1.0
                }
            }
        ],
        userZoomingEnabled: true,
        userPanningEnabled: true
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
    const fixCheckbox = document.getElementById("fix-option");
    const report = document.getElementById("fix-report");

    generateButton.addEventListener('click', () => {
        errorOutput.innerText = "";
        matrixContainer.innerHTML = "";
        cy.innerHTML = "";
        report.innerHTML = "";

        const rowSums = parseInputString(rowInput.value);
        const colSums = parseInputString(colInput.value);

        try {
            const matrix = generate_matrix_wasm(rowSums, colSums, fixCheckbox.checked);

            const newRowSums = matrix.map(row => row.reduce((a, b) => a + b, 0));
            const newColSums = matrix[0].map((_, colIndex) => 
                matrix.reduce((sum, row) => sum + row[colIndex], 0)
            );

            if (fixCheckbox.checked) {
                if (rowSums.join(',') !== newRowSums.join(',') || colSums.join(',') !== newColSums.join(',')) {
                    report.innerText = `Matrix Fixed. \nNew Row Sums: [${newRowSums.join(', ')}]\nNew Column Sums: [${newColSums.join(', ')}]`;
                } else {
                    report.innerText = "No fixes needed. Matrix was already valid";
                }
            }
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