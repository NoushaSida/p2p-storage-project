import bcrypt

class Hasher:
    """
    This class will hash plain text passwords.
    """
    def __init__(self, passwords: list):
        """
        Create a new instance of "Hasher".

        Parameters
        ----------
        passwords: list
            The list of plain text passwords to be hashed.
        """
        self.passwords = passwords

    def _gen_salt():
        return bcrypt.gensalt()

    def _hash(self, password: str, salt) -> str:
        """
        Hashes the plain text password.

        Parameters
        ----------
        password: str
            The plain text password to be hashed.
        Returns
        -------
        str
            The hashed password.
        """
        return bcrypt.hashpw(password.encode(), salt).decode()

    def generate(self, salt=None) -> list:
        """
        Hashes the list of plain text passwords.

        Returns
        -------
        list
            The list of hashed passwords.
        """
        if not salt:
            salt=bcrypt.gensalt()
        return [self._hash(password, salt) for password in self.passwords]