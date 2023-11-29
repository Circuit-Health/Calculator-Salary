# Calculator-Salary
 Calculate your Australian salary and taxes

curl -X POST http://127.0.0.1:3000/calculate_tax \
     -H "Content-Type: application/json" \
     -d '{"salary": 50000, "year": 2023, "calculate_beyond_max": false}'

