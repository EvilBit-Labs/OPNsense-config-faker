from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Snatrules(BaseModel):
    class Meta:
        name = "snatrules"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
