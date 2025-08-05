from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Jostletimeout(BaseModel):
    class Meta:
        name = "jostletimeout"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
