from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Rules(BaseModel):
    class Meta:
        name = "rules"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
