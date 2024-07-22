// Parse Tree
// To store data in the tree, we follow certain rules based on the tree's structure.
// These rules are:
// If the current symbol is (, a new node is added as the left child node, and we descend to that node.
// If the current symbol is one of "+", "-", "*", or "/", we set the root value to that symbol, 
// add a new right child node, and descend to the right child node.
// If the current symbol is a number, we set the root value to that number and return to the parent node.
