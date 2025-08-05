from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Lists(BaseModel):
    class Meta:
        name = "lists"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
