from ractors import CSV

csv = CSV(file_path="example.csv", delimiter=";")
df = csv.read()