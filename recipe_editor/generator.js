addEventListener("load", () => {
    //Create the initial row for the table
    addRow()

    //Appends a new row to the table
    document.getElementById("newRowBtn").addEventListener("click", addRow)
    //When <Enter> is pressed, add another row as well
    document.addEventListener("keyup", (event) => {
        if(event.code == "Enter") addRow()
    })
    //Exports the form's data to JSON, ready to be used in the program
    document.getElementById("generateRecipeBtn").addEventListener("click", generateRecipe)
})

//Used to generate the unit of measurement selection options
const units = {
    "Cups": "Cups",
    "Ounces": "Ounces",
    "Tablespoons": "Tablespoons",
    "Teaspoons": "Teaspoons",
    "Pinch": "Pinch",
    "Whole": "Whole"
}

function addRow() {
    let table = document.getElementById("ingredients")

    //Generate a "unique" ID for the new row. Used to group elements for programmatic access
    let idx = Math.floor(Math.random() * 200000)
    let row = table.insertRow()
    row.id = `entry|${idx}` //Used in `deleteRow`

    //Generate the inputs for this row:
    //Ingredient name field
    let ingName = row.insertCell()
    let ingNameInput = document.createElement("input")
    ingNameInput.id = `ingredient|${idx}`
    ingNameInput.type = "text"
    ingName.appendChild(ingNameInput)

    //Ingredient quantity field
    let ingAmount = row.insertCell()
    let ingAmountInput = document.createElement("input")
    ingAmountInput.id = `quantity|${idx}`
    ingAmountInput.type = "text"
    ingAmount.appendChild(ingAmountInput)

    //Units of measurement selection field
    let unit = row.insertCell()
    let unitInput = document.createElement("select")
    unitInput.id = `unit|${idx}`
    for (const unit in units) {
        let opt = document.createElement("option")
        opt.setAttribute("value", unit)
        let text = document.createTextNode(units[unit])
        opt.appendChild(text)
        unitInput.appendChild(opt)
    }
    unit.appendChild(unitInput)

    let deleteRow = row.insertCell()
    let deleteRowBtn = document.createElement("input")
    deleteRowBtn.type = "button"
    deleteRowBtn.value = "X"
    deleteRowBtn.id = `delete|${idx}`
    deleteRowBtn.addEventListener("click", () => {
        removeRow(idx)
    })
    deleteRow.appendChild(deleteRowBtn)
}

//Delete a row from the table using the generated ID to find the specific row
//If the row is the last data row in the table, the row will not be removed (UI concerns)
function removeRow(idx) {
    let table = document.getElementById("ingredients")
    if(table.rows.length == 2) return

    for(const rowIdx in table.rows) {
        let row = table.rows[rowIdx]
        if(!row.id.startsWith("entry")) continue
        let rowId = row.id.split("|")[1]
        if(rowId != idx) continue

        table.deleteRow(rowIdx)
        return
    }
}

//Read the table's rows and export to JSON!
function generateRecipe() {
    let table = document.getElementById("ingredients")
    // if(table.rows.length == 2) return

    let recipe = {}
    let name = document.getElementById("recipeName").value
    if(name == null || name.trim() == "") return
    recipe["name"] = name
    let ingredients = []
    
    for(const row of table.rows) {
        if(!row.id.startsWith("entry")) continue
        let idx = row.id.split("|")[1]

        let ingName = document.getElementById(`ingredient|${idx}`).value.trim()
        let ingQty = Number(document.getElementById(`quantity|${idx}`).value.trim())
        let ingUnit = document.getElementById(`unit|${idx}`).value

        if(ingName == "" || ingQty == "" || ingQty == null) continue

        let measure = {
            "quantity": ingQty,
            "unit": ingUnit,
        }

        ingredients.push({
            "name": ingName,
            "measure": measure
        })
    }

    recipe["ingredients"] = ingredients
    download(recipe)
}

//https://stackoverflow.com/a/34156339
function download(recipe) {
    let a = document.createElement("a")
    let file = new Blob([JSON.stringify(recipe)], {type: "text/plain"})
    a.href = URL.createObjectURL(file)
    a.download = `${recipe.name}.json`
    a.click()
    URL.revokeObjectURL(a.href)
}