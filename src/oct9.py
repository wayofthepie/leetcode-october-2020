# This challenge has no rust version yet!

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Codec:
    def serialize(self, root: TreeNode) -> str:
        """Encodes a tree to a single string."""
        if root is None:
            return "[]"
        nums = self._level_order(root)
        for i in range(len(nums) - 1, 0, -1):
            if nums[i] == None:
                nums.pop()
            else:
                break
        return str(nums).replace("None", "null").replace(" ", "")

    def _level_order(self, root: TreeNode):
        nums = []
        q = []
        q.append(root)
        while len(q) > 0:
            current = q.pop(0)
            if current is None:
                nums.append(current)
                continue
            nums.append(current.val)
            q.append(current.left)
            q.append(current.right)
        return nums

    def deserialize(self, data: str) -> TreeNode:
        nodes = list(
            map(
                lambda num: None if num == "null" or num == "" else TreeNode(int(num)),
                data.strip("[]").split(","),
            )
        )
        if not nodes:
            return None
        reverse = nodes[::-1]
        root = reverse.pop()
        for node in nodes:
            if node:
                if reverse:
                    node.left = reverse.pop()
                if reverse:
                    node.right = reverse.pop()
        return root
