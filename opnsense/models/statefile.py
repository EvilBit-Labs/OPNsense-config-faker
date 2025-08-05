from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Statefile(BaseModel):
    class Meta:
        name = "statefile"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
