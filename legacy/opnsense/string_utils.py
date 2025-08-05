"""
Shared string utilities for OPNsense configuration generation.

This module provides common string escaping functionality used across
multiple modules to ensure consistent behavior.
"""

from xml.sax.saxutils import escape
from typing import Dict, Optional


# Standard HTML entity mappings for XML escaping
HTML_ENTITY_MAPPINGS = {
    "ä": "&#xE4;", "ö": "&#xF6;", "ü": "&#xFC;", "ß": "&#xDF;",
    "Ä": "&#xC4;", "Ö": "&#xD6;", "Ü": "&#xDC;"
}

# ASCII replacement mappings for sanitization
ASCII_MAPPINGS = {
    "ä": "ae", "ö": "oe", "ü": "ue", "ß": "ss",
    "Ä": "AE", "Ö": "OE", "Ü": "UE"
}

# Extended sanitization mappings (includes ASCII + additional chars)
SANITIZE_MAPPINGS = {
    **ASCII_MAPPINGS,
    " ": "", "-": "_", "/": "_"
}


def escape_string(s: str, mappings: Optional[Dict[str, str]] = None) -> str:
    """
    Escape a string using XML-safe character mappings.

    Args:
        s: The string to escape
        mappings: Optional custom character mappings. If None, uses HTML entities.

    Returns:
        The escaped string
    """
    if mappings is None:
        mappings = HTML_ENTITY_MAPPINGS

    return escape(s, mappings)


def escape_html_entities(s: str) -> str:
    """
    Escape a string using HTML entity references.

    Args:
        s: The string to escape

    Returns:
        The string with HTML entity references
    """
    return escape_string(s, HTML_ENTITY_MAPPINGS)


def sanitize_string(s: str) -> str:
    """
    Sanitize a string by replacing special characters with ASCII equivalents
    and removing/transforming problematic characters.

    Args:
        s: The string to sanitize

    Returns:
        The sanitized string
    """
    return escape_string(s, SANITIZE_MAPPINGS)
