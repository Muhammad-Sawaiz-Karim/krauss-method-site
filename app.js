import init, { generate_matrix_wasm, generate_fulkerson_wasm } from './pkg/rust_port.js';

function parseInputString(inputString) {
    const stringArray = inputString.split(',');
    const numberArray = stringArray.map(str => parseInt(str.trim(), 10));
    return new Int32Array(numberArray.filter(num => !isNaN(num)));
}

function drawMatrix(matrixArray) {
    let latexString = "\\begin{bmatrix}\n";
    for (let i = 0; i < matrixArray.length; i++) {
        latexString += matrixArray[i].join(" & ") + " \\\\\n";
    }
    latexString += "\\end{bmatrix}";
    katex.render(latexString, document.getElementById("matrix-container"), {
        displayMode: true, throwOnError: false
    });
}

// GRAPH 1: The Bipartite Graph (Krauss)
function drawKraussGraph(matrixArray) {
    const elements = [];
    const rowCount = matrixArray.length;
    const colCount = matrixArray[0].length;
    const horizontalSpacing = 100, verticalGap = 250; 

    for (let i = 0; i < rowCount; i++) {
        elements.push({ data: { id: `R${i}`, label: `R${i}` }, position: { x: i * horizontalSpacing, y: 0 }, classes: 'row-node' });
    }
    
    const offset = ((rowCount - 1) * horizontalSpacing - (colCount - 1) * horizontalSpacing) / 2;
    for (let i = 0; i < colCount; i++) {
        elements.push({ data: { id: `C${i}`, label: `C${i}` }, position: { x: (i * horizontalSpacing) + offset, y: verticalGap }, classes: 'col-node' });
    }

    for (let i = 0; i < rowCount; i++) {
        for (let j = 0; j < colCount; j++) {
            if (matrixArray[i][j] == 1) elements.push({ data: { id: `E-R${i}-C${j}`, source: `R${i}`, target: `C${j}` } });
        }
    }

    cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        layout: { name: 'preset', padding: 50, fit: true },
        style: [
            { selector: 'node', style: { 'label': 'data(label)', 'color': '#fff', 'text-valign': 'center', 'text-halign': 'center', 'font-family': 'monospace', 'width': '40px', 'height': '40px' } },
            { selector: '.row-node', style: { 'background-color': '#342f97', 'shape': 'round-rectangle' } },
            { selector: '.col-node', style: { 'background-color': '#0c8b61', 'shape': 'ellipse' } },
            { selector: 'edge', style: { 'width': 2, 'line-color': '#2f3236', 'curve-style': 'bezier', 'opacity': 0.8 } }
        ]
    });
}

// GRAPH 2: The Directed Graph (Fulkerson)
function drawFulkersonGraph(matrixArray) {
    const elements = [];
    const nodeCount = matrixArray.length; 

    for (let i = 0; i < nodeCount; i++) {
        elements.push({ data: { id: `N${i}`, label: `N${i}` } });
    }

    for (let i = 0; i < nodeCount; i++) {
        for (let j = 0; j < nodeCount; j++) {
            if (matrixArray[i][j] == 1) elements.push({ data: { id: `E-${i}-${j}`, source: `N${i}`, target: `N${j}` } });
        }
    }

    cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        layout: { name: 'circle', padding: 50, fit: true },
        style: [
            { selector: 'node', style: { 'label': 'data(label)', 'color': '#fff', 'text-valign': 'center', 'text-halign': 'center', 'font-family': 'monospace', 'width': '40px', 'height': '40px', 'background-color': '#342f97' } },
            { selector: 'edge', style: { 'width': 2, 'line-color': '#2f3236', 'curve-style': 'bezier', 'target-arrow-shape': 'triangle', 'target-arrow-color': '#2f3236', 'arrow-scale': 1.5, 'opacity': 0.8 } }
        ]
    });
}

async function run() {
    await init();
    console.log("WASM Loaded Successfully!");
    
    const generateButton = document.getElementById("generate_button");
    const methodSelect = document.getElementById("method-select");
    const fixWrapper = document.getElementById("fix-wrapper");
    const fixCheckbox = document.getElementById("fix-option");
    
    // UI Logic: Hide the "Fix" checkbox if Fulkerson is selected
    methodSelect.addEventListener('change', (e) => {
        if (e.target.value === 'fulkerson') {
            fixWrapper.style.display = 'none';
        } else {
            fixWrapper.style.display = 'flex';
        }
    });

    generateButton.addEventListener('click', () => {
        document.getElementById("error-output").innerText = "";
        document.getElementById("fix-report").innerText = "";
        document.getElementById("matrix-container").innerHTML = "";
        document.getElementById("cy").innerHTML = ""; 

        const rowSums = parseInputString(document.getElementById("row-input").value);
        const colSums = parseInputString(document.getElementById("col-input").value);
        const method = methodSelect.value;

        try {
            if (method === 'krauss') {
                const matrix = generate_matrix_wasm(rowSums, colSums, fixCheckbox.checked);
                
                // Reporting Logic (Krauss only)
                if (fixCheckbox.checked) {
                    const newRowSums = matrix.map(row => row.reduce((a, b) => a + b, 0));
                    const newColSums = matrix[0].map((_, colIndex) => matrix.reduce((sum, row) => sum + row[colIndex], 0));
                    if (rowSums.join(',') !== newRowSums.join(',') || colSums.join(',') !== newColSums.join(',')) {
                        document.getElementById("fix-report").innerText = `Matrix Fixed. \nNew Row Sums: [${newRowSums.join(', ')}]\nNew Column Sums: [${newColSums.join(', ')}]`;
                    } else {
                        document.getElementById("fix-report").innerText = "No fixes needed. Matrix was already valid";
                    }
                }
                
                drawMatrix(matrix);
                drawKraussGraph(matrix);
                
            } else if (method === 'fulkerson') {
                const matrix = generate_fulkerson_wasm(rowSums, colSums);
                drawMatrix(matrix);
                drawFulkersonGraph(matrix);
            }
        }
        catch (error) {
            console.error("Error: ", error);
            document.getElementById("error-output").innerText = error;
        }
    });
}
run();