from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Logreplies(BaseModel):
    class Meta:
        name = "logreplies"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
