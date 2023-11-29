document.getElementById('salaryForm').addEventListener('submit', function(event) {
    event.preventDefault();

    const salary = document.getElementById('salary').value;
    const year = document.getElementById('year').value;
    const calculateBeyondMax = document.getElementById('calculateBeyondMax').checked;

    fetch('http://127.0.0.1:3000/calculate_tax', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ salary, year, calculate_beyond_max: calculateBeyondMax }),
    })
    .then(response => response.json())
    .then(data => {
        document.getElementById('results').textContent = `Annual Post-Tax Salary: ${data.annual_post_tax_salary}, Superannuation: ${data.superannuation}`;
    })
    .catch((error) => {
        console.error('Error:', error);
    });
});
