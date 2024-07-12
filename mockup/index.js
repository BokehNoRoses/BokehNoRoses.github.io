var counter = 0;

function showTooltip(html) {
	var tooltip = document.getElementById("tooltip");
	tooltip.innerHTML = html;
	tooltip.style.visibility = 'visible';
}

function hideTooltip() {
	var tooltip = document.getElementById("tooltip");
	tooltip.innerHTML = "";
	tooltip.style.visibility = 'hidden';
}

function updateTerm(val) {
	switch(parseInt(val, 10)) {
		case 0:
			document.getElementById("term-body").innerHTML = "<h3>placeholder0</h3>";
			break;
		case 1:
			document.getElementById("term-body").innerHTML = "<h3>placeholder1</h3>";
			break;
		case 2:
			document.getElementById("term-body").innerHTML = "<h3>placeholder2</h3>";
			break;
		case 3:
			document.getElementById("term-body").innerHTML = "<h3>placeholder3</h3>";
			break;
		case 4:
			document.getElementById("term-body").innerHTML = "<h3>placeholder4</h3>";
			break;
	}
}

function updateCarousel(val) {
	switch(parseInt(val, 10)) {
		case 0:
			counter = 0;
			document.getElementById("c-synop").innerHTML = "a";
			break;
		case 1:
			counter = 1;
			document.getElementById("c-synop").innerHTML = "b";
			break;
		case 2:
			counter = 2;
			document.getElementById("c-synop").innerHTML = "c";
			break;
		case 3:
			counter = 3;
			document.getElementById("c-synop").innerHTML = "d";
			break;
		case 4:
			counter = (counter + 3) % 4;
			updateCarousel(counter);
			document.getElementById("c-btn-" + counter).checked = true;
			break;
		case 5:
			counter = (counter + 1) % 4;
			updateCarousel(counter);
			document.getElementById("c-btn-" + counter).checked = true;
			break;
	}
}

// window.addEventListener("load", () => {});
