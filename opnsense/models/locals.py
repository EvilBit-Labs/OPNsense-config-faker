from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Locals(BaseModel):
    class Meta:
        name = "locals"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
