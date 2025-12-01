from typing import List, Dict, Union

def search(root_path: str, pattern: str) -> List[Dict[str, Union[str, int]]]:
    """
    Search recursively for a regex pattern, respecting .gitignore.
    Returns a list of dicts: {"path": str, "line": int, "content": str}
    """
    ...

