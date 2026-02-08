import re
import constants as c

class Validator:
    """
    This class will check the validity of the entered username, name, and email for a 
    newly registered user.
    """
    def validate_username(self, username: str) -> bool:
        """
        Checks the validity of the entered username.

        Parameters
        ----------
        username: str
            The usernmame to be validated.
        Returns
        -------
        bool
            Validity of entered username.
        """
        pattern = r"^[a-zA-Z0-9_-]{1,20}$"
        return bool(re.match(pattern, username))

    def validate_name(self, name: str) -> bool:
        """
        Checks the validity of the entered name.
        
        Parameters
        ----------
        name: str
            The name to be validated.
        Returns
        -------
        bool
            Validity of entered name.
        """
        return 1 < len(name) < 100

    def validate_mount_point(self, folder: str) -> bool:
        """
        Checks the validity of the entered name.
        
        Parameters
        ----------
        folder: str
            The name to be validated.
        Returns
        -------
        bool
            Validity of entered name.
        """
        return bool(re.compile(r'^/([a-zA-Z0-9_\-]+/?)+$').match(folder))

    def validate_disk_size(self, size: str) -> bool:
        """
        Checks the validity of the entered name.
        
        Parameters
        ----------
        name: str
            The name to be validated.
        Returns
        -------
        bool
            Validity of entered name.
        """
        if size.isdigit():
            disk_size = int(size)
            return c.DISK_SIZE_MB_MIN <= disk_size <= c.DISK_SIZE_MB_MAX 
        return False

    def validate_disk_type(self, type: str) -> bool:
        """
        Checks the validity of the entered name.
        
        Parameters
        ----------
        type: str
            The type to be validated.
        Returns
        -------
        bool
            Validity of entered name.
        """
        return type in c.STORAGE_TYPE

    def validate_email(self, email: str) -> bool:
        """
        Checks the validity of the entered email.

        Parameters
        ----------
        email: str
            The email to be validated.
        Returns
        -------
        bool
            Validity of entered email.
        """
        return "@" in email and 2 < len(email) < 320