---

marp: true
theme: default


------

<style>
.red { color: red; }
.green { color: green; }
.blue { color: blue; }
.orange { color: orange; }
.purple { color: purple; }
.black { color: black; }

td {
  width: 40px;
  text-align: center;
  font-size: 12px;
}

.grid {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.cell {
  width: 30%;
  min-width: 260px;
}
</style>

# Azul

<p align="center">
  <img src="https://cf.geekdo-images.com/LLj6LLz7RvE6bcgxFV3d_g__imagepage/img/q03K851_67Ot7son3ateepTrl34=/fit-in/900x600/filters:no_upscale():strip_icc()/pic3864358.jpg" width="300" height="auto">
</p>




---

# My Inspiration

<p align="center">
  <img src="https://cf.geekdo-images.com/IwzIeSMhoCWq5M6ZjC7oWw__original/img/xTNtnN8LwV66OqzanA2ULFbLLv0=/0x0/filters:format(png)/pic7975235.png" width="300" height="auto">
</p>

---

# Tree Search Monte Carlo Simulation

* Embarrassingly parallel
* Rapidly growing state space

---

# Finding an Approach

* Extreme scaling challenge
<div style="
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 20px;
    flex-wrap: wrap;
">
    <img src="desmos-graph-5.png" alt="Graph 1" style="max-width: 20%; height: auto;" />
    <img src="desmos-graph-6.png" alt="Graph 2" style="max-width: 20%; height: auto;" />
</div>


---

# My Approach

* 2 tiles per row, no duplicate colors
* Breadth-first search
<p align="center">
  <img src="https://cf.geekdo-images.com/LLj6LLz7RvE6bcgxFV3d_g__imagepage/img/q03K851_67Ot7son3ateepTrl34=/fit-in/900x600/filters:no_upscale():strip_icc()/pic3864358.jpg" width="300" height="auto">
</p>

---

# Parallelization Method

* BFS supports parallel frontier expansion
* Independent nodes for each level
* Work distribution across threads

---

# Rayon vs OpenMP

* Rayon: work-stealing, dynamic load balancing
* OpenMP: static/dynamic scheduling options
* Memory model differences
* Ease of nested parallelism
* Rust safety vs C/C++ control

---

# MPI + Rayon Hybrid

* MPI for first-level branching
* Rayon for deeper parallelism
<img src="image-1.png" alt="Graph 1" style="max-width: 45%; height: auto;" />

---

# Results

(Insert data visuals here)

---



### Scoring Progression

<div class="grid">

<div class="cell">

**Step 1 — Score: 2**

|  |  |  |  |  |  |
|---|---|---|---|---|---|
| <span class="green"><b>1/1</b></span> |   | <span class="green"><b>X</b></span> |   |   |   |
| <span class="orange"><b>2/2</b></span> |   |   |   |   | <span class="orange"><b>X</b></span> |
| <span class="red">2/3</span> |   |   |   |   |   |
| <span class="purple">2/4</span> |   |   |   |   |   |
| <span class="blue">2/5</span> |   |   |   |   |   |

</div>


<div class="cell">

**Step 2 — Score: 10**

|  |  |  |  |  |  |
|---|---|---|---|---|---|
| <span class="orange"><b>1/1</b></span> |   | <span class="green"><b>O</b></span> |   | <span class="orange"><b>X</b></span> |   |
| <span class="blue"><b>2/2</b></span> |   |   |   | <span class="blue"><b>X</b></span> | <span class="orange"><b>O</b></span> |
| <span class="red"><b>3/3</b></span> |   |   | <span class="red"><b>X</b></span> |   |   |
| <span class="purple"><b>4/4</b></span> |   |   | <span class="purple"><b>X</b></span> |   |   |
| <span class="blue">2/5</span> |   |   |   |   |   |

</div>
</div>

---
<div class="grid">
<div class="cell">

**Step 3 — Score: 18**

|  |  |  |  |  |  |
|---|---|---|---|---|---|
| <span class="red"><b>1/1</b></span> | <span class="red"><b>X</b></span> | <span class="green"><b>O</b></span> |   | <span class="orange"><b>O</b></span> |   |
| <span class="green"><b>2/2</b></span> |   |   | <span class="green"><b>X</b></span> | <span class="blue"><b>O</b></span> | <span class="orange"><b>O</b></span> |
| <span class="purple">2/3</span> |   |   | <span class="red"><b>O</b></span> |   |   |
| <span class="orange">2/4</span> |   |   | <span class="purple"><b>O</b></span> |   |   |
| <span class="blue">4/5</span> |   |   |   |   |   |

</div>


<div class="cell">

**Step 4 — Score: 43**

|  |  |  |  |  |  |
|---|---|---|---|---|---|
| <span class="blue"><b>1/1</b></span> | <span class="red"><b>O</b></span> | <span class="green"><b>O</b></span> | <span class="blue"><b>X</b></span> | <span class="orange"><b>O</b></span> |   |
| <span class="red"><b>2/2</b></span> |   | <span class="red"><b>X</b></span> | <span class="green"><b>O</b></span> | <span class="blue"><b>O</b></span> | <span class="orange"><b>O</b></span> |
| <span class="purple"><b>3/3</b></span> |   | <span class="purple"><b>X</b></span> | <span class="red"><b>O</b></span> |   |   |
| <span class="orange"><b>4/4</b></span> |   | <span class="orange"><b>X</b></span> | <span class="purple"><b>O</b></span> |   |   |
| <span class="blue">4/5</span> |   |   |   |   |   |

</div>
</div>

---

<div class="grid">
<div class="cell">

**Step 5 — Score: 55**

|  |  |  |  |  |  |
|---|---|---|---|---|---|
| <span class="purple"><b>1/1</b></span> | <span class="red"><b>O</b></span> | <span class="green"><b>O</b></span> | <span class="blue"><b>O</b></span> | <span class="orange"><b>O</b></span> | <span class="purple"><b>X</b></span> |
| <span class="purple">0/2</span> |   | <span class="red"><b>O</b></span> | <span class="green"><b>O</b></span> | <span class="blue"><b>O</b></span> | <span class="orange"><b>O</b></span> |
| <span class="green">2/3</span> |   | <span class="purple"><b>O</b></span> | <span class="red"><b>O</b></span> |   |   |
| <span class="red">2/4</span> |   | <span class="orange"><b>O</b></span> | <span class="purple"><b>O</b></span> |   |   |
| <span class="blue"><b>5/5</b></span> |   | <span class="blue"><b>X</b></span> |   |   |   |

</div>


<div class="cell">

**Step 6 — Score: 75**

|  |  |  |  |  |  |
|---|---|---|---|---|---|
| <span class="red">0/1</span> | <span class="red"><b>O</b></span> | <span class="green"><b>O</b></span> | <span class="blue"><b>O</b></span> | <span class="orange"><b>O</b></span> | <span class="purple"><b>O</b></span> |
| <span class="purple"><b>2/2</b></span> | <span class="purple"><b>X</b></span> | <span class="red"><b>O</b></span> | <span class="green"><b>O</b></span> | <span class="blue"><b>O</b></span> | <span class="orange"><b>O</b></span> |
| <span class="green"><b>3/3</b></span> |   | <span class="purple"><b>O</b></span> | <span class="red"><b>O</b></span> | <span class="green"><b>X</b></span> |   |
| <span class="red"><b>4/4</b></span> |   | <span class="orange"><b>O</b></span> | <span class="purple"><b>O</b></span> | <span class="red"><b>X</b></span> |   |
| <span class="orange">2/5</span> |   | <span class="blue"><b>O</b></span> |   |   |   |

</div>

</div>


---

# Questions
