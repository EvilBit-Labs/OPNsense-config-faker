from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Children(BaseModel):
    class Meta:
        name = "children"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
