addEventListener("load", () => {
    //Create the initial row for the table
    addRow()

    //Appends a new row to the table
    document.getElementById("newRowBtn").addEventListener("click", addRow)
    //When <Enter> is pressed, add another row as well
    document.addEventListener("keyup", (event) => {
        if(event.code == "Enter") addRow(false)
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

function addRow(focusInput) {
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
    ingNameInput.tabIndex = 0
    ingName.appendChild(ingNameInput)

    //Ingredient quantity field
    let ingAmount = row.insertCell()
    let ingAmountInput = document.createElement("input")
    ingAmountInput.id = `quantity|${idx}`
    ingAmountInput.type = "text"
    ingAmountInput.tabIndex = 0
    ingAmount.appendChild(ingAmountInput)

    //Units of measurement selection field
    let unit = row.insertCell()
    let unitInput = document.createElement("select")
    unitInput.id = `unit|${idx}`
    unitInput.tabIndex = 0
    for (const unit in units) {
        let opt = document.createElement("option")
        opt.setAttribute("value", unit)
        let text = document.createTextNode(units[unit])
        opt.appendChild(text)
        unitInput.appendChild(opt)
    }
    //When the user presses "<Tab>" on this field, check
    //if this is the last row in the table. If it is,
    //add another row and give focus to the new ingredient name field.
    unitInput.addEventListener("blur", (event) => {
        let table = document.getElementById("ingredients")
        let lastRow = table.rows[table.rows.length - 1]
        if(lastRow == null) return

        let id = lastRow.id
        if(id.indexOf(idx) == -1) return

        addRow(true)
    })
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

    if(focusInput) {
        ingNameInput.focus()
    }
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
    
    let name = document.getElementById("recipeName").value
    if(name == null || name.trim() == "") return
    let recipe = {
        "name": name,
        "ingredients": []
    }
    
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

        recipe.ingredients.push({
            "name": ingName,
            "measure": measure
        })
    }

    download(recipe)
}

//https://stackoverflow.com/a/34156339
function download(recipe) {
    let a = document.createElement("a")
    let file = new Blob([JSON.stringify(recipe)], {type: "text/plain;charset=utf-8"})
    a.href = URL.createObjectURL(file)
    a.download = `${recipe.name}.json`
    a.click()
    URL.revokeObjectURL(a.href)
}