from oct9 import Codec, TreeNode
import unittest


class TestBinaryTree(unittest.TestCase):
    def test_simple_tree_serialization(self):
        root = TreeNode(4)
        root.left = TreeNode(2)
        root.left.left = TreeNode(1)
        root.left.right = TreeNode(3)
        root.right = TreeNode(6)
        root.right.left = TreeNode(5)
        ser = Codec()
        self.assertEqual(ser.serialize(root), "[4,2,6,1,3,5]")

    def test_simple_tree_deserialization(self):
        root = TreeNode(4)
        root.left = TreeNode(2)
        root.left.left = TreeNode(1)
        root.left.right = TreeNode(3)
        root.right = TreeNode(6)
        root.right.left = TreeNode(5)
        des = Codec()
        tree = des.deserialize("[4,2,6,1,3,5]")
        self.assertEqual(tree.val, root.val)
        self.assertEqual(tree.left.val, root.left.val)
        self.assertEqual(tree.left.left.val, root.left.left.val)
        self.assertEqual(tree.left.right.val, root.left.right.val)
        self.assertEqual(tree.right.val, root.right.val)
        self.assertEqual(tree.right.left.val, root.right.left.val)

    def test_small_tree_deserialization(self):
        root = TreeNode(2)
        root.left = TreeNode(1)
        root.right = TreeNode(3)
        des = Codec()
        tree = des.deserialize("[2,1,3]")
        self.assertEqual(tree.val, root.val)
        self.assertEqual(tree.left.val, root.left.val)
        self.assertEqual(tree.right.val, root.right.val)

    def test_single_node_tree_deserialization(self):
        root = TreeNode(2)
        des = Codec()
        tree = des.deserialize("[2]")
        self.assertEqual(tree.val, root.val)

    def test_single_node_tree_serialization(self):
        root = TreeNode(2)
        codec = Codec()
        tree_str = codec.serialize(root)
        self.assertEqual(tree_str, "[2]")

    def test_empty_node_serialization(self):
        codec = Codec()
        tree_str = codec.serialize(None)
        self.assertEqual(tree_str, "[]")

    def test_empty_string_deserialization(self):
        codec = Codec()
        tree = codec.deserialize("[]")
        self.assertEqual(tree, None)

    def test_isomorphic(self):
        codec = Codec()
        given = "[4,2,6,1,3,5]"
        tree = codec.deserialize(given)
        tree_str = codec.serialize(tree)
        self.assertEqual(given, tree_str)

    def test_isomorphic_with_null(self):
        de = Codec()
        se = Codec()
        given = "[1,null,2]"
        tree = de.deserialize(given)
        tree_str = se.serialize(tree)
        self.assertEqual(given, tree_str)

    def test_isomorphic_with_null_serialize_then_deserialize(self):
        root = TreeNode(1)
        root.left = None
        root.right = TreeNode(2)
        se = Codec()
        de = Codec()
        tree_str = se.serialize(root)
        tree = de.deserialize(tree_str)
        self.assertEqual(tree.val, root.val)
        self.assertEqual(tree.left, root.left)

    def test_isomorphic_with_multi_null_serialize_then_deserialize(self):
        # [5,3,6,2,4,null,null,1]
        root = TreeNode(5)
        root.left = TreeNode(3)
        root.right = TreeNode(6)
        root.left.left = TreeNode(2)
        root.left.right = TreeNode(4)
        root.right.left = None
        root.right.right = None
        root.left.left.left = TreeNode(1)
        se = Codec()
        de = Codec()
        tree_str = se.serialize(root)
        tree = de.deserialize(tree_str)
        self.assertEqual(tree.val, root.val)
        self.assertEqual(tree.right.left, root.right.left)
        self.assertEqual(tree.right.right, root.right.right)
