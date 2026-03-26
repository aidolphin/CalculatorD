#!/bin/bash

# Build frontend
echo "Building frontend WebAssembly..."
cd frontend
wasm-pack build --target web --out-dir pkg
cd ..

# Create static directories
mkdir -p static/pkg
mkdir -p static/css
mkdir -p static/js

# Copy frontend files
cp -r frontend/pkg/* static/pkg/

# Create minimal HTML if needed
cat > static/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Calculator.ai - Free Online Calculators</title>
    <link rel="stylesheet" href="/css/styles.css">
    <script type="module">
        import init from '/pkg/frontend.js';
        init().catch(console.error);
    </script>
</head>
<body>
    <div id="app"></div>
</body>
</html>
EOF

# Create basic CSS
cat > static/css/styles.css << 'EOF'
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
}

.navbar {
    background: white;
    padding: 1rem 2rem;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.nav-brand h1 {
    color: #4a90e2;
    font-size: 1.5rem;
}

.nav-links {
    display: flex;
    gap: 2rem;
    margin-top: 0.5rem;
    flex-wrap: wrap;
}

.nav-links a {
    text-decoration: none;
    color: #2c3e50;
    font-weight: 500;
}

.nav-links a:hover {
    color: #4a90e2;
}

main {
    max-width: 1200px;
    margin: 2rem auto;
    padding: 0 1rem;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 2rem;
}

.calculator-card {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}

.calculator-card h2 {
    color: #4a90e2;
    margin-bottom: 1rem;
}

.input-group {
    margin-bottom: 1rem;
}

.input-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #2c3e50;
}

.input-group input, .input-group select {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #ecf0f1;
    border-radius: 8px;
    font-size: 1rem;
}

.input-group input:focus {
    outline: none;
    border-color: #4a90e2;
}

button {
    width: 100%;
    padding: 0.75rem;
    background: #4a90e2;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
}

button:hover {
    background: #357abd;
}

.result {
    margin-top: 1rem;
    padding: 1rem;
    background: #ecf0f1;
    border-radius: 8px;
}

.result h3 {
    color: #4a90e2;
    margin-bottom: 0.5rem;
}

@media (max-width: 768px) {
    main {
        grid-template-columns: 1fr;
    }
}
EOF

echo "Build complete! Run 'cargo run' to start the server"
