function copyToClipboard() {
  let copyText = document.getElementById("message")

  navigator.clipboard.writeText(copyText.innerText);

  console.log(copyText.innerText);
  alert("Message copier!")
}
